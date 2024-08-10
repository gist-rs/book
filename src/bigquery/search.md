# Search

### How to [search](https://cloud.google.com/bigquery/docs/reference/standard-sql/search_functions#search)

```sql
SELECT id, content FROM `project-a.dataset-b.table-c`
WHERE SEARCH(content, '線形回帰')
```

### How to do multi-language text inference in `BigQuery`

```sql
CREATE OR REPLACE MODEL `llm.text_inference_model`
  REMOTE WITH CONNECTION `project-a.asia-northeast1.connection-b`
  OPTIONS (ENDPOINT = 'gemini-1.5-flash-preview-0514');
```

### How to perform vector search in BigQuery

```sql
CREATE OR REPLACE TABLE
  content_hub_v2.poc ( query_id STRING NOT NULL,
    embedding ARRAY<FLOAT64> );
INSERT
  content_hub_v2.poc (query_id,
    embedding)
VALUES
  ('線形回帰', (
    SELECT
      ARRAY_AGG(text_embeddings) AS text_embeddings
    FROM
      ML.GENERATE_EMBEDDING( MODEL `llm.text_embedding_model`,
        (
        SELECT
          "線形回帰" AS content)),
      UNNEST(ml_generate_embedding_result)AS text_embeddings ));
SELECT
  *
FROM
  VECTOR_SEARCH( TABLE content_hub_v2.poc,
    'embedding',
    (
    SELECT
      ARRAY_AGG(embedding) AS embedding
    FROM
      ML.GENERATE_EMBEDDING( MODEL `llm.text_embedding_model`,
        (
        SELECT
          "線形回帰" AS content)),
      UNNEST(ml_generate_embedding_result)AS embedding ),
    'embedding',
    top_k => 2);
```
