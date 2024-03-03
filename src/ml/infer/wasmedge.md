# WasmEdge

## Install WASI-NN with GGML Backend

```bash
# Install everything (recommended)
bash <(curl -sSfL 'https://code.flows.network/webhook/iwYN1SdN3AmPgR5ao5Gt/run-llm.sh')
source ~/.zshenv

# Or only plugin https://wasmedge.org/docs/start/install/#install-wasmedge-with-plug-ins
curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash -s -- --plugins wasmedge_rustls wasi_nn-ggml
```

## Models

```bash
curl -LO https://huggingface.co/TheBloke/Mistral-7B-Instruct-v0.1-GGUF/resolve/main/mistral-7b-instruct-v0.1.Q5_K_M.gguf
```

## API Server/Chat (optional)

```
curl -LO https://github.com/second-state/LlamaEdge/releases/latest/download/llama-api-server.wasm
curl -LO https://github.com/second-state/LlamaEdge/releases/latest/download/llama-chat.wasm
```

## M3Max

```bash
wasmedge --dir .:. --nn-preload default:GGML:AUTO:mistral-7b-instruct-v0.1.Q5_K_M.gguf llama-api-server.wasm -p chatml -r '<|im_end|>' -s 0.0.0.0:8081
```

RAM `4.1GB`

## Thai

```
# Model OpenThaiGPT
curl -LO https://huggingface.co/openthaigpt/openthaigpt-1.0.0-beta-13b-chat-gguf/resolve/main/ggml-model-q4_0.gguf

# Or typhoon (need --ctx-size 10000 but not working due to no instruct)
curl -LO https://huggingface.co/TheBloke/typhoon-7B-GGUF/resolve/main/typhoon-7b.Q4_K_M.gguf

# Or SeaLLM
curl -LO https://huggingface.co/parinzee/SeaLLM-7B-Chat-GGUF/resolve/main/seallm-7b-chat.q4_k_m.gguf

# Chat CLI
wasmedge --dir .:. --nn-preload default:GGML:AUTO:ggml-model-q4_0.gguf llama-chat.wasm --log-stat

# API
wasmedge --dir .:. --nn-preload default:GGML:AUTO:ggml-model-q4_0.gguf llama-api-server.wasm

# API - test
curl -X POST http://0.0.0.0:8080/v1/chat/completions -H 'accept:application/json' -H 'Content-Type: application/json' -d '{"messages":[{"role":"system", "content":"You are a helpful AI assistant"}, {"role":"user", "content":"กทม ย่อมาจากอะไร"}], "model":"openthaigpt-1.0.0-beta-13b-chat"}'

# Chat GUI
curl -LO https://github.com/second-state/chatbot-ui/releases/download/v0.1.0/chatbot-ui.tar.gz
tar xzf chatbot-ui.tar.gz
wasmedge --dir .:. --nn-preload default:GGML:AUTO:ggml-model-q4_0.gguf llama-api-server.wasm -p llama-2-chat
```

## Sea

```
curl -LO https://huggingface.co/parinzee/SeaLLM-7B-Chat-GGUF/resolve/main/seallm-7b-chat.q4_k_m.gguf
wasmedge --dir .:. --nn-preload default:GGML:AUTO:seallm-7b-chat.q4_k_m.gguf llama-api-server.wasm
```

---

## Windows

> Ref: https://github.com/second-state/WasmEdge-WASINN-examples

```bash
wasmedge --dir .:. --env n_gpu_layers=35 --nn-preload default:GGML:AUTO:mistral-7b-instruct-v0.1.Q5_K_M.gguf wasmedge-ggml-llama-interactive.wasm default
```

RAM `7012MiB / 24564MiB`

### With `lama-chat`

> Ref: https://github.com/second-state/llama-utils

> mistral-7b-instruct-v0.1.Q5_K_M

```bash
wasmedge --dir .:. --nn-preload default:GGML:AUTO:mistral-7b-instruct-v0.1.Q5_K_M.gguf llama-chat.wasm -p mistral-instruct-v0.1 -r '</s>'
```

RAM `9608MiB / 24564MiB`

> mistrallite.Q5_K_M

```bash
wasmedge --dir .:. --nn-preload default:GGML:AUTO:mistrallite.Q5_K_M.gguf llama-chat.wasm -p mistrallite -r '</s>'
```

RAM `9608MiB / 24564MiB`

### With `llama-api-server`

> mistrallite.Q5_K_M

```bash
wasmedge --dir .:. --nn-preload default:GGML:AUTO:mistrallite.Q5_K_M.gguf llama-api-server.wasm -p mistrallite -r '</s>'
```

> openhermes-2.5-mistral-7b.Q5_K_M

```bash
curl -LO https://huggingface.co/second-state/OpenHermes-2.5-Mistral-7B-GGUF/resolve/main/openhermes-2.5-mistral-7b.Q5_K_M.gguf

wasmedge --dir .:. --nn-preload default:GGML:AUTO:openhermes-2.5-mistral-7b.Q5_K_M.gguf llama-api-server.wasm -p chatml -r '<|im_end|>'

# Or 8081
wasmedge --dir .:. --nn-preload default:GGML:AUTO:openhermes-2.5-mistral-7b.Q5_K_M.gguf llama-api-server.wasm -p chatml -r '<|im_end|>' -s 0.0.0.0:8081
```

### Test

```bash
curl -X POST http://localhost:8080/v1/chat/completions -H 'accept:application/json' -H 'Content-Type: application/json' -d '{"messages":[{"role":"system", "content": "You are a helpful assistant."}, {"role":"user", "content": "Write helloworld code in Rust"}], "model":"MistralLite-7B"}'

# Or 8081
curl -X POST http://localhost:8081/v1/chat/completions -H 'accept:application/json' -H 'Content-Type: application/json' -d '{"messages":[{"role":"system", "content": "You are a helpful assistant."}, {"role":"user", "content": "Write helloworld code in Rust"}], "model":"openhermes-2.5-mistral-7b.Q5_K_M"}'
```

RAM `23862MiB / 24564MiB`
