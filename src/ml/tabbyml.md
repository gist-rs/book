# TabbyML

## How to run `TabbyML`

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

1. Optional code completion model use `TabbyML/CodeLlama-7B`

   ```bash
   sudo docker run -it --gpus all -p 8080:8080 -v $HOME/.tabby:/data tabbyml/tabby serve --model TabbyML/Mistral-7B --device cuda
   ```

1. Optional chat model use `TabbyML/Mistral-7B`

   > ⚠️ I can't make this one work, it's just crash and exit. 🤔

   ```bash
   sudo docker run -it --gpus all -p 8080:8080 -v $HOME/.tabby:/data tabbyml/tabby serve --model TabbyML/StarCoder-1B --chat-model TabbyML/Mistral-7B --device cuda
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

- `CLI` Health report
- `CLI` Model selection
- `CLI` Configurable repos
- `CLI` Indexing trigger
- `Query` Include/Exclude repos for faster query
- `Query` Include/Exclude language for faster query
- `Embedding` Code in comment, PDF, Table, Image
- `Extension` Chat with Tabby
