## LoRA

```bash
# Setup
venv .venv
source .venv/bin/activate
pip install mlx-lm

# Config
# MODEL=SeaLLMs/SeaLLM-7B-v2
# MODEL=Qwen/Qwen1.5-7B
# MODEL=Qwen/Qwen1.5-7B-Chat
# MODEL=sail/Sailor-7B-Chat
MODEL=meta-llama/Llama-3.2-3B-Instruct

# Infer
mlx_lm.generate \
    --model ${MODEL} \
    --eos-token "<|eot_id|>" \
    --max-tokens 500 \
    --prompt "K Point คืออะไร ?"

# Train LoRA
mlx_lm.lora \
    --model ${MODEL} \
    --train \
    --data /Users/katopz/git/mlx-examples/lora/data \
    --iters 500

# Train DoRA
mlx_lm.lora \
    --model ${MODEL} \
    --train \
    --data /Users/katopz/git/mlx-examples/lora/data \
    --fine-tune-type dora
    --iters 500

# Test
mlx_lm.lora \
    --model ${MODEL} \
    --adapter-path adapters \
    --data /Users/katopz/git/mlx-examples/lora/data \
    --test

# Generate
mlx_lm.generate \
    --model ${MODEL} \
    --adapter-path adapters \
    --max-tokens 256 \
    --prompt "K Point คืออะไร ?"

# Fused
mlx_lm.fuse \
    --model ${MODEL}

# Upload (after huggingface-cli login with writeable token)
mlx_lm.fuse \
    --model ${MODEL} \
    --upload-repo katopz/kbtg-kpoint-v1-fused \
    --hf-path ${MODEL}

# GGUF
mlx_lm.fuse \
    --model ${MODEL} \
    --export-gguf

# Infer local fused
mlx_lm.generate \
    --model ./lora_fused_model \
    --eos-token "<|eot_id|>" \
    --max-tokens 256 \
    --prompt "K Point คืออะไร ?"

# Infer with ollama
ollama run hf.co/katopz/kbtg-kpoint-v1-fused
```
