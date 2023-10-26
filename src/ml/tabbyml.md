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

1. Optional use `TabbyML/CodeLlama-7B`

   ```bash
   sudo docker run -it --gpus all -p 8080:8080 -v $HOME/.tabby:/data tabbyml/tabby serve --model TabbyML/CodeLlama-7B --device cuda
   ```

## How to get code completion = index from target repos

1. Optional schedule now

   > Refer to https://tabby.tabbyml.com/blog/2023/10/16/repository-context-for-code-completion/

   ```bash
   sudo docker run -v $HOME/.tabby:/data tabbyml/tabby scheduler --now
   ```

## How to request the `TabbyML` services from other machine to Windows WSL2

1. Open Windows firewall and create new rule that allow port 8080.

1. See your host info

   ```bash
   wsl hostname -I
   ipconfig
   ```

1. Then forward port 8080 to WLS2
   ```bash
   netsh interface portproxy add v4tov4 listenaddress=192.168.1.33 listenport=8080 connectaddress=127.0.0.1 connectport=8080
   ```
1. Open http://192.168.1.33:8080 in your browser.
