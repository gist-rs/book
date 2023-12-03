# WasmEdge

## Only `wasmedge`
> Ref: https://github.com/second-state/WasmEdge-WASINN-examples
```bash
wasmedge --dir .:. --env n_gpu_layers=35 --nn-preload default:GGML:AUTO:mistral-7b-instruct-v0.1.Q5_K_M.gguf wasmedge-ggml-llama-interactive.wasm default
```
RAM `7012MiB / 24564MiB`

## With `lama-chat`
> Ref: https://github.com/second-state/llama-utils

> mistral-7b-instruct-v0.1.Q5_K_M
```bash
curl -LO https://github.com/second-state/llama-utils/raw/main/chat/llama-chat.wasm

wasmedge --dir .:. --nn-preload default:GGML:AUTO:mistral-7b-instruct-v0.1.Q5_K_M.gguf llama-chat.wasm -p mistral-instruct-v0.1 -r '</s>'
```
RAM `9608MiB / 24564MiB`

> mistrallite.Q5_K_M
```bash
curl -LO https://github.com/second-state/llama-utils/raw/main/chat/llama-chat.wasm

wasmedge --dir .:. --nn-preload default:GGML:AUTO:mistrallite.Q5_K_M.gguf llama-chat.wasm -p mistrallite -r '</s>'
```
RAM `9608MiB / 24564MiB`

## With `llama-api-server`
> mistrallite.Q5_K_M
```bash
curl -LO https://github.com/second-state/llama-utils/raw/main/api-server/llama-api-server.wasm

wasmedge --dir .:. --nn-preload default:GGML:AUTO:mistrallite.Q5_K_M.gguf llama-api-server.wasm -p mistrallite -r '</s>'
```

> openhermes-2.5-mistral-7b.Q5_K_M
```bash
curl -LO https://huggingface.co/second-state/OpenHermes-2.5-Mistral-7B-GGUF/resolve/main/openhermes-2.5-mistral-7b.Q5_K_M.gguf

wasmedge --dir .:. --nn-preload default:GGML:AUTO:openhermes-2.5-mistral-7b.Q5_K_M.gguf llama-api-server.wasm -p chatml -r '<|im_end|>'
```

## Test
```bash
curl -X POST http://localhost:8080/v1/chat/completions -H 'accept:application/json' -H 'Content-Type: application/json' -d '{"messages":[{"role":"system", "content": "You are a helpful assistant."}, {"role":"user", "content": "Write helloworld code in Rust"}], "model":"MistralLite-7B"}'
```

RAM `23862MiB / 24564MiB`
