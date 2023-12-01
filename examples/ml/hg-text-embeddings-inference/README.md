https://github.com/huggingface/text-embeddings-inference

```bash
model=BAAI/bge-large-en-v1.5
revision=refs/pr/5

 # share a volume with the Docker container to avoid downloading weights every run
volume=$PWD/data

docker run --gpus all -p 8083:80 -v $volume:/data --pull always ghcr.io/huggingface/text-embeddings-inference:0.4.0 --model-id $model --revision $revision
```

```bash
curl 127.0.0.1:8083/embed \
    -X POST \
    -d '{"inputs":"What is Deep Learning?"}' \
    -H 'Content-Type: application/json'
```
