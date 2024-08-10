# Inferences

## Setup for simple use case.

1. [Create and set up a Cloud resource connection.](https://cloud.google.com/bigquery/docs/create-cloud-resource-connection)
2. Target Vertex AI LLM (see below)

### How to do multi-language text embedding in `BigQuery`

> You can select model from [here](https://cloud.google.com/vertex-ai/generative-ai/docs/learn/models)

```sql
 CREATE OR REPLACE MODEL `llm.text_inference_model`
  REMOTE WITH CONNECTION `project-a.asia-northeast1.connection-b`
  OPTIONS (ENDPOINT = 'gemini-1.5-flash-preview-0514');
```

### How to infer via BigQuery and return with `JSON` struct

```sql
SELECT
  TO_JSON_STRING(STRUCT(
    ml_generate_text_llm_result AS result
  )) AS json_output
FROM
  ML.GENERATE_TEXT(MODEL `llm.text_inference_model`,
    (
      SELECT
        CONCAT('''Hello World''', content) AS prompt,
        *
      FROM
        `project-a.asia-northeast1.connection-b`
      WHERE
        id = "baz"
    ),
    STRUCT(
      0.5 AS temperature,
      TRUE AS flatten_json_output
    )
  );
```
