# llama.cpp

## Infer homebrew

> Recommend for user.

```bash
brew install llama.cpp

REPO=Qwen/Qwen2-1.5B-Instruct-GGUF
FILE=qwen2-1_5b-instruct-q4_k_m.gguf
PROMPT="Write helloworld code in Rust"
llama-cli --hf-repo ${REPO} --hf-file ${FILE} -p "${PROMPT}" -i --n-gpu-layers 10
```

## Build

> Recommend for dev.

```bash
git clone https://github.com/ggml-org/llama.cpp.git
cd llama.cpp

# not working: build but no GPU
make GGML_CUDA=1

# not working: get an error `cicc: not found`
cmake -B build -DGGML_BLAS=ON -DGGML_BLAS_VENDOR=OpenBLAS -DLLAMA_CUBLAS=ON

# working after `rm -rf build`
cmake -B build -DCMAKE_CUDA_COMPILER=/usr/local/cuda-12.4/bin/nvcc -DGGML_BLAS=ON -DGGML_BLAS_VENDOR=OpenBLAS -DGGML_CUDA=ON

cmake --build build --config Release

# copy to current dir
cp ./build/bin/* .

# but still slow
CUDACXX=/usr/local/cuda-12.4/bin/nvcc CMAKE_ARGS="-DLLAMA_CUBLAS=on -DCMAKE_CUDA_ARCHITECTURES=all-major" FORCE_CMAKE=1 pip install llama-cpp-python --no-cache-dir --force-reinstall --upgrade

# try again
cmake -B build -DCMAKE_CUDA_COMPILER=/usr/local/cuda-12.4/bin/nvcc -DGGML_BLAS=ON -DGGML_BLAS_VENDOR=OpenBLAS -DGGML_CUDA=ON -DCMAKE_CUDA_ARCHITECTURES=all-major

FORCE_CMAKE=1 cmake --build build --config Release

# copy to current dir
cp ./build/bin/* .

# but still slow...🤔
```

## Prepare model

```bash
curl -LO https://huggingface.co/second-state/gemma-2-9b-it-GGUF/resolve/main/gemma-2-9b-it-Q4_K_M.gguf
```

## Infer

```bash
MODEL=gemma-2-9b-it-Q4_K_M.gguf
PROMPT="Write helloworld code in Rust"
./llama-cli -m ${MODEL} -p "${PROMPT}" -i --n-gpu-layers 10
```

## Infer - Docker

```bash
MODEL=second-state/gemma-2-9b-it-GGUF
docker run --gpus all -v ./:/models local/llama.cpp:full-cuda --run -m ./${MODEL} -p "Write helloworld code in Rust" -n 512 --n-gpu-layers 1
```
