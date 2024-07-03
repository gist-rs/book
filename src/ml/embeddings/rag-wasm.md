lightblue/suzume-llama-3-8B-multilingual

MODEL=rinna/nekomata-7b-instruction-gguf
MODEL=CohereForAI/aya-23-8B
MODEL=mlx-community/aya-23-8B-4bit
python -m mlx_lm.generate --model ${MODEL} --max-tokens 512 --prompt "Translate to English: おはようございます"

lmstudio-community/aya-23-8B-GGUF

# Completions

## Install WasmEdge

```bash
curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash -s -- --plugins wasi_nn-ggml
```

## Load Chat Model

```bash
curl -LO https://huggingface.co/lmstudio-community/aya-23-8B-GGUF/resolve/main/aya-23-8B-Q5_K_M.gguf
```

## Setup Server

```bash
curl -LO https://github.com/LlamaEdge/LlamaEdge/releases/latest/download/llama-api-server.wasm
wasmedge --dir .:. --nn-preload default:GGML:AUTO:aya-23-8B-Q5_K_M.gguf \
  llama-api-server.wasm \
  --prompt-template chatml \
  --ctx-size 16384 \
  --model-name aya-23-8B-Q5_K_M
```

## Test

```bash
curl -X POST http://localhost:8080/v1/chat/completions \
  -H 'accept:application/json' \
  -H 'Content-Type: application/json' \
  -d '{"messages":[{"role":"user", "content": "Translate おはようございます to English."}], "model":"aya-23-8B-Q5_K_M"}'
```

# Embbeding

## Load Embedding Model

```bash
curl -LO https://huggingface.co/gaianet/Nomic-embed-text-v1.5-Embedding-GGUF/resolve/main/nomic-embed-text-v1.5.f16.gguf
```

## Setup VectorDB

```bash
mkdir qdrant_storage
mkdir qdrant_snapshots

nohup docker run -d -p 6333:6333 -p 6334:6334 \
    -v $(pwd)/qdrant_storage:/qdrant/storage:z \
    -v $(pwd)/qdrant_snapshots:/qdrant/snapshots:z \
    qdrant/qdrant
```

## Test VectorDB

```bash
curl -X DELETE 'http://localhost:6333/collections/default'

curl -X PUT 'http://localhost:6333/collections/default' \
  -H 'Content-Type: application/json' \
  --data-raw '{
    "vectors": {
      "size": 768,
      "distance": "Cosine",
      "on_disk": true
    }
  }'
```

## Setup Embedding

```bash
curl -LO https://github.com/GaiaNet-AI/embedding-tools/raw/main/markdown_embed/markdown_embed.wasm
```

## Setup Data

```bash
curl -LO https://huggingface.co/datasets/gaianet/paris/raw/main/paris.md
```

## Embedding

```bash
wasmedge --dir .:. \
  --nn-preload embedding:GGML:AUTO:nomic-embed-text-v1.5.f16.gguf \
  markdown_embed.wasm embedding default 768 paris.md --heading_level 1 --ctx_size 8192
```
