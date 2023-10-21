# TabbyML

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

1. Optional use `TabbyML/CodeLlama-7B`

```bash
sudo docker run -it --gpus all -p 8080:8080 -v $HOME/.tabby:/data tabbyml/tabby serve --model TabbyML/CodeLlama-7B --device cuda
```
