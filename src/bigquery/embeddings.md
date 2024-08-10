# Embeddings

![](/assets/kat.png) <span class="speech-bubble">Let's do embeddings! The most challenge is chunking to match the max token input which [depend on each model](https://cloud.google.com/vertex-ai/generative-ai/docs/learn/models).</span>

## Setup for simple use case.

1. [Create and set up a Cloud resource connection.](https://cloud.google.com/bigquery/docs/create-cloud-resource-connection)
2. Target Vertex AI LLM (see below)

### How to do multi-language text embedding in `BigQuery`?

```sql
CREATE OR REPLACE MODEL `llm.text_embedding_model`
  REMOTE WITH CONNECTION `project-a.asia-northeast1.connection-b`
  OPTIONS (ENDPOINT = 'text-multilingual-embedding-002');
```

### How to query `ml_generate_embedding_result` as `ARRAY<FLOAT64>`?

```sql
SELECT ARRAY_AGG(text_embeddings) AS text_embeddings
FROM ML.GENERATE_EMBEDDING(
    MODEL `llm.text_embedding_model`,
    (SELECT "線形回帰" AS content)
),
UNNEST(ml_generate_embedding_result) AS text_embeddings
```

### How can we know token length before embedding?

```sql
WITH analyzed_text AS (
    SELECT ARRAY_LENGTH(TEXT_ANALYZE('Hello World! This is so fun!')) AS token_count
)
SELECT JSON_OBJECT(
    'truncated', token_count > 5,
    'token_count', token_count
) AS result
FROM analyzed_text;
```

Output

```json
{ "token_count": 6, "truncated": true }
```

### How can we know token length before embedding for each chunks?

```sql
WITH input_chunks AS (
    SELECT chunk
    FROM UNNEST(['foo bar baz', 'foo', 'foo bar']) AS chunk
)
SELECT ARRAY(
    SELECT ARRAY_LENGTH(TEXT_ANALYZE(chunk))
    FROM input_chunks
) AS token_counts;
```

Output

```json
[3, 1, 2]
```

## Setup for `chunking` update.

1. [Create sessions](https://cloud.google.com/bigquery/docs/sessions-intro) for `TEMP` table.
2. Update with extracted result (see below)

### How to update `ml_generate_embedding_result` and also get `ml_generate_embedding_statistics`?

```sql
CREATE OR REPLACE TEMP TABLE temp_embedding_stats AS
  SELECT ml_generate_embedding_result AS text_embeddings, ml_generate_embedding_statistics AS statistics, "foo" AS id
  FROM ML.GENERATE_EMBEDDING(
    MODEL `llm.text_embedding_model`,
    (
      SELECT
        "foo" AS title,
        "bar" AS content
    ),
    STRUCT(TRUE AS flatten_json_output, 'RETRIEVAL_DOCUMENT' as task_type)
  );

UPDATE `project-a.dataset-b.table-c`
SET
text_embeddings = ARRAY(
  SELECT text_embeddings
  FROM UNNEST(
    (SELECT text_embeddings FROM `temp_embedding_stats` WHERE id = "foo")
  ) AS text_embeddings
),
statistics = (SELECT statistics FROM `temp_embedding_stats` WHERE id = "foo")
WHERE id = "foo";
```

---

## Not working approach (note to self)

### BigQuery can't use `WITH` with `UPDATE`

> `Syntax error: Unexpected keyword UPDATE at [26:1]`

```sql
WITH existing_embeddings AS (
  SELECT text_embeddings
  FROM `project-a.dataset-b.table-c`
  WHERE id = "baz"
),
new_embedding AS (
  SELECT ml_generate_embedding_result, ml_generate_embedding_statistics
  FROM ML.GENERATE_EMBEDDING(
    MODEL `llm.text_embedding_model`,
    (
      SELECT
        "foo" AS title,
        "bar" AS content
    ),
    STRUCT(TRUE AS flatten_json_output, 'RETRIEVAL_DOCUMENT' as task_type)
  )
),
combined_embeddings AS (
  SELECT embedding
  FROM UNNEST((SELECT text_embeddings FROM existing_embeddings)) AS embedding
  UNION ALL
  SELECT embedding
  FROM new_embedding, UNNEST(ml_generate_embedding_result) AS embedding
)

UPDATE `project-a.dataset-b.table-c`
SET text_embeddings = ARRAY(SELECT embedding FROM combined_embeddings)
WHERE id = "baz";
```

### BigQuery can't use `USING` with `UPDATE`

> `Table "S" must be qualified with a dataset (e.g. dataset.table).`

```sql
MERGE `project-a.dataset-b.table-c` T
USING (
    SELECT ml_generate_embedding_result, ml_generate_embedding_statistics
    FROM ML.GENERATE_EMBEDDING(
      MODEL `llm.text_embedding_model`,
      (
        SELECT
          "foo" AS title,
          "bar" AS content
      ),
      STRUCT(TRUE AS flatten_json_output, 'RETRIEVAL_DOCUMENT' as task_type)
    )
  ) S
ON T.id = S.id
WHEN MATCHED THEN
UPDATE
SET text_embeddings = ARRAY(
  SELECT embedding
  FROM UNNEST(
    (SELECT text_embeddings FROM `project-a.dataset-b.table-c` WHERE id = "baz")
  ) AS embedding
  UNION ALL
  SELECT embedding
  FROM S, UNNEST(ml_generate_embedding_result) AS embedding
)
WHEN NOT MATCHED THEN
  INSERT (id, created_at, updated_at, title, content, title)
  VALUES (
    "baz",
    CURRENT_DATETIME(),
    CURRENT_DATETIME(),
    "foo",
    "bar"
  );

```
