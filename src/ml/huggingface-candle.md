# 🤗 HuggingFace - Candle

> [https://github.com/huggingface/candle]()

A minimalist ML framework for Rust with a focus on performance (including GPU support) and ease of use.

### Examples

These online demos run entirely in your browser:

- [yolo](https://huggingface.co/spaces/lmz/candle-yolo): pose estimation and
  object recognition.
- [whisper](https://huggingface.co/spaces/lmz/candle-whisper): text to speech.
- [LLaMA2](https://huggingface.co/spaces/lmz/candle-llama2): text generation.
- [LLaMA and LLaMA-v2](https://github.com/huggingface/candle/blob/main/candle-examples/examples/llama/): general LLM.
- [Falcon](https://github.com/huggingface/candle/blob/main/candle-examples/examples/falcon/): general LLM.
- [StarCoder](https://github.com/huggingface/candle/blob/main/candle-examples/examples/bigcode/): LLM specialized to code
  generation.
- [Quantized LLaMA](https://github.com/huggingface/candle/blob/main/candle-examples/examples/quantized/): quantized version of
  the LLaMA model using the same quantization techniques as
  [llama.cpp](https://github.com/ggerganov/llama.cpp).

<img src="https://github.com/huggingface/candle/raw/main/candle-examples/examples/quantized/assets/aoc.gif" width="600">
  
- [Stable Diffusion](https://github.com/huggingface/candle/blob/main/candle-examples/examples/stable-diffusion/): text to
  image generative model, support for the 1.5, 2.1, and SDXL 1.0 versions.

<img src="https://github.com/huggingface/candle/raw/main/candle-examples/examples/stable-diffusion/assets/stable-diffusion-xl.jpg" width="200">

- [yolo-v3](https://github.com/huggingface/candle/blob/main/candle-examples/examples/yolo-v3/) and
  [yolo-v8](https://github.com/huggingface/candle/blob/main/candle-examples/examples/yolo-v8/): object detection and pose
  estimation models.

<img src="https://github.com/huggingface/candle/raw/main/candle-examples/examples/yolo-v8/assets/bike.od.jpg" width="200"><img src="https://github.com/huggingface/candle/raw/main/candle-examples/examples/yolo-v8/assets/bike.pose.jpg" width="200">

- [segment-anything](https://github.com/huggingface/candle/blob/main/candle-examples/examples/segment-anything/): image
  segmentation model with prompt.

<img src="https://github.com/huggingface/candle/raw/main/candle-examples/examples/segment-anything/assets/sam_merged.jpg" width="200">

- [Whisper](https://github.com/huggingface/candle/blob/main/candle-examples/examples/whisper/): speech recognition model.
- [Bert](https://github.com/huggingface/candle/blob/main/candle-examples/examples/bert/): useful for sentence embeddings.
- [DINOv2](https://github.com/huggingface/candle/blob/main/candle-examples/examples/dinov2/): computer vision model trained
  using self-supervision (can be used for imagenet classification, depth
  evaluation, segmentation).

### Setup

![](/assets/kat.png) <span class="speech-bubble">[llama2-c](https://github.com/huggingface/candle/blob/main/candle-wasm-examples/llama2-c/README.md) on Windows 11 WSL2 Ubuntu, RTX4090</span>

```bash
# Choose example llama2-c
cd candle-wasm-examples/llama2-c

# Setup raw
wget https://huggingface.co/spaces/lmz/candle-llama2/resolve/main/model.bin
wget https://huggingface.co/spaces/lmz/candle-llama2/resolve/main/tokenizer.json

# Setup tools
cargo install --locked trunk
cargo install --locked wasm-bindgen-cli
sudo apt install libssl-dev

# Build
. ./build-lib.sh

# Serve
trunk serve --release --port 8081
open http://127.0.0.1:8081
```

![](/assets/kat.png) <span class="speech-bubble">[Mistral](https://mistral.ai/) on Windows 11 WSL2 Ubuntu, RTX4090</span>

```bash
# Update & upgrade
sudo apt update && sudo apt upgrade

# Remove previous NVIDIA installation
sudo apt autoremove nvidia* --purge

# Setup cuda ref: https://gist.github.com/denguir/b21aa66ae7fb1089655dd9de8351a202
wget https://developer.download.nvidia.com/compute/cuda/repos/ubuntu2204/x86_64/cuda-ubuntu2204.pin
sudo mv cuda-ubuntu2204.pin /etc/apt/preferences.d/cuda-repository-pin-600
wget https://developer.download.nvidia.com/compute/cuda/12.2.0/local_installers/cuda-repo-ubuntu2204-12-2-local_12.2.0-535.54.03-1_amd64.deb
sudo dpkg -i cuda-repo-ubuntu2204-12-2-local_12.2.0-535.54.03-1_amd64.deb
sudo cp /var/cuda-repo-ubuntu2204-12-2-local/cuda-*-keyring.gpg /usr/share/keyrings/
sudo apt-get -y install cuda

sudo ubuntu-drivers autoinstall
sudo apt install nvidia-driver-525
sudo apt install nvidia-cuda-toolkit
sudo apt-get -y install cuda-nvcc-12-2
sudo apt-get -y install cuda-toolkit-12-2

# Check NVIDIA Drivers
nvidia-smi

# Check CUDA
nvcc --version

# Setup cuDNN
sudo apt install libcudnn8
sudo apt install libcudnn8-dev

# Check CuDNN
/sbin/ldconfig -N -v $(sed 's/:/ /' <<< $LD_LIBRARY_PATH) 2>/dev/null | grep libcudnn

# Setup raw
wget https://huggingface.co/lmz/candle-mistral/resolve/main/pytorch_model-00001-of-00002.safetensors
wget https://huggingface.co/lmz/candle-mistral/resolve/main/pytorch_model-00002-of-00002.safetensors
wget https://huggingface.co/lmz/candle-mistral/resolve/main/tokenizer.json

# Source
echo 'export CUDA_HOME=/usr/local/cuda-12.2' >> ~/.bashrc
echo 'export PATH=/usr/local/cuda-12.2/bin:$PATH' >> ~/.bashrc
echo 'export LD_LIBRARY_PATH=/usr/local/cuda-12.2/lib64:$LD_LIBRARY_PATH' >> ~/.bashrc
source ~/.bashrc
sudo ln -s /usr/local/cuda-12.2/bin/nvcc /usr/bin/nvcc

# Run
cargo run --example mistral --release --features cuda,cudnn -- --prompt "Write helloworld code in Rust" --weight-files=pytorch_model-00001-of-00002.safetensors,pytorch_model-00002-of-00002.safetensors --tokenizer-file=tokenizer.json --sample-len 15
```
