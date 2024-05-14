# TabbyML

## How to install for `Apple Silicon` via `homebrew`

> Ref: https://tabby.tabbyml.com/docs/installation/apple

```
# Install homebrew
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install tabby
brew install tabbyml/tabby/tabby

# Serve completion and chat with some model https://tabby.tabbyml.com/docs/models/
tabby serve --device metal --model TabbyML/DeepseekCoder-6.7B --chat-model TabbyML/Mistral-7B

# Or
tabby serve --device metal --model TabbyML/Mistral-7B --chat-model TabbyML/Mistral-7B

# Or with port
tabby serve --device metal --model TabbyML/CodeGemma-7B --chat-model TabbyML/CodeGemma-7B-Instruct --port 9090

# Config to indexing some repo
cat << 'EOF' > ~/.tabby/config.toml
[[repositories]]
git_url = "https://github.com/rust-lang/book.git"

[[repositories]]
git_url = "https://github.com/rust-lang/rust-by-example.git"
EOF

# Indexing now
tabby scheduler --now
```

## How to add more repos manually

### Open config file

```
open ~/.tabby/config.toml
```

### Then add some repo

```
[[repositories]]
git_url = "https://github.com/DioxusLabs/dioxus.git"
```

### Finally force indexing

```
tabby scheduler --now
```

## RAM used

- `--model TabbyML/DeepseekCoder-6.7B` // RAM used 2.1GB
- `--chat-model TabbyML/Mistral-7B` // RAM used 2.6GB

---

## How to run `TabbyML` via `Windows`

1. Install `Docker` via `WSL`

   ```bash
   sudo docker run -it --gpus all -p 8080:8080 -v $HOME/.tabby:/data tabbyml/tabby serve --model TabbyML/StarCoder-1B --device cuda
   ```

   And you will get an error.

   ```bash
   docker: Error response from daemon: could not select device driver "" with capabilities: [[gpu]].
   ```

   The [fix](https://github.com/NVIDIA/nvidia-docker/issues/1243#issuecomment-928064024)

   ```bash
   distribution=$(. /etc/os-release;echo $ID$VERSION_ID)
   curl -s -L https://nvidia.github.io/nvidia-docker/gpgkey | sudo apt-key add -
   curl -s -L https://nvidia.github.io/nvidia-docker/$distribution/nvidia-docker.list | sudo tee /etc/apt/sources.list.d/nvidia-docker.list

   sudo apt-get update && sudo apt-get install -y nvidia-container-toolkit
   sudo systemctl restart docker
   ```

## How to select model

1. Optional code completion model use `TabbyML/CodeLlama-7B`

   ```bash
   sudo docker run -it --gpus all -p 8080:8080 -v $HOME/.tabby:/data tabbyml/tabby serve --model TabbyML/CodeLlama-7B --device cuda
   ```

1. Optional code completion model use `TabbyML/Mistral-7B`

   ```bash
   sudo docker run -it --gpus all -p 8080:8080 -v $HOME/.tabby:/data tabbyml/tabby serve --model TabbyML/Mistral-7B --device cuda
   ```

1. Optional chat model use `TabbyML/Mistral-7B`

   > ⚠️ I can't make this one work, it's just crash and exit. 🤔

   ```bash
   sudo docker run -it --gpus all -p 8080:8080 -v $HOME/.tabby:/data tabbyml/tabby serve --model TabbyML/StarCoder-1B --chat-model TabbyML/Mistral-7B --device cuda
   ```

## How to build and run docker locally to match your cuda version e.g. 12.3.0

```
docker build --build-arg CUDA_VERSION=12.3.0 -t tabby_cuda12_3 .
docker run -it --gpus all -p 8080:8080 -v $HOME/.tabby:/data tabby_cuda12_3 serve --device cuda --model TabbyML/StarCoder-1B --chat-model TabbyML/Mistral-7B
```

## How to get code completion = index from target repos

1. Optional schedule now

   > Refer to https://tabby.tabbyml.com/blog/2023/10/16/repository-context-for-code-completion/

   ```bash
   sudo docker run -v $HOME/.tabby:/data tabbyml/tabby scheduler --now
   ```

1. Or schedule via running docker
   ```bash
   sudo docker ps -a | grep tabby | awk '{print $1}' | xargs sudo docker exec -it $1 sh -c "/opt/tabby/bin/tabby scheduler --now"
   ```

## How to request the `TabbyML` services from other machine to Windows WSL2

1. See your host info

   ```bash
   wsl hostname -I
   ipconfig
   ```

1. Open `Windows Firewall`→`Advanced Settings` and create new `Inbound Rules` for `Your local IP4 (e.g. 192.168.1.33)` that allow port `8080`.
1. Then forward port `8080` to `WLS2`
   ```bash
   netsh interface portproxy add v4tov4 listenaddress=192.168.1.33 listenport=8080 connectaddress=127.0.0.1 connectport=8080
   ```
1. Open in your browser.
   ```bash
   open http://192.168.1.33:8080
   ```

## Ideas

- `CLI` Lazy git.
- `CLI` Auto fix after compile.
- `CLI` Model selection.
- `CLI` Configurable repos.
- `CLI` Indexing manual trigger.
- `Query` Include/Exclude repos for faster query.
- `Query` Include/Exclude language for faster query.
- `Embedding` Code in comment?, PDF, Table, Image.

---

## How to dev tabby

```
# Setup
git clone --recurse-submodules https://github.com/TabbyML/tabby
cd tabby

# macos
brew install protobuf
brew install cmake

# Update
git pull
git submodule update --init --recursive
```

## How to build

> ref: https://github.com/rust-lang/rust/issues/117976

```
## Workaround for Rust 1.17.4
rustup default nightly

## Build release
cargo build --release
```
