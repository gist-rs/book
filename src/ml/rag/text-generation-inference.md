# Text Generation Inference

## Zephyr.md
> Ref: https://github.com/huggingface/text-generation-inference#docker

### Run
```
model=HuggingFaceH4/zephyr-7b-beta
volume=$PWD/data

docker run --gpus all --shm-size 1g -p 8080:80 -v $volume:/data ghcr.io/huggingface/text-generation-inference:1.2 --model-id $model
```

---

## MistralLite - tgi (failed)
> Ref: https://github.com/awslabs/extending-the-context-length-of-open-source-llms

### Issue 1
> On `WSL2` if you got 
`ERROR: failed to solve: ghcr.io error getting credentials` 

Ref: https://stackoverflow.com/a/71665244

1. Open 
    ```
    code ~/.docker/config.json
    ```
2. Delete `"credsStore": "desktop.exe"`
    ```
    {
        "credsStore": "desktop.exe"
    }
    ```
3. For some reason you have to do this every time.

### Issue 2
> Ref: https://github.com/huggingface/text-generation-inference/issues/451

> On `WSL2` there is somehow hopeless.

```
2023-12-02 17:03:55 2023-12-02T10:03:55.996765Z  WARN text_generation_launcher: No safetensors weights found for model amazon/MistralLite at revision None. Converting PyTorch weights to safetensors.
2023-12-02 17:03:55 
2023-12-02 17:04:18 Error: DownloadError
2023-12-02 17:04:18 2023-12-02T10:04:18.068001Z ERROR download: text_generation_launcher: Download process was signaled to shutdown with signal 9: 
```

---

## MistralLite - hg
> hg-transformers: https://github.com/awslabs/extending-the-context-length-of-open-source-llms/blob/9bd948000c25a1f34a56657a21ad6b2a167f73ea/MistralLite/huggingface-transformers/example_usage.ipynb

RAM `16215MiB / 24564MiB`

> tgi: https://github.com/awslabs/extending-the-context-length-of-open-source-llms/blob/8b2608196ceb291ac788c6ed0ca06aa1a0d20cb0/MistralLite/tgi/example_usage.ipynb

RAM `23097MiB / 24564MiB`
