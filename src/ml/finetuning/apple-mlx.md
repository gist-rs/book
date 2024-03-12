## LoRA

```bash
# Setup
python3 -m venv .venv
source .venv/bin/activate
pip install mlx-lm

# Config
# MODEL=SeaLLMs/SeaLLM-7B-v2
# MODEL=Qwen/Qwen1.5-7B
# MODEL=Qwen/Qwen1.5-7B-Chat
MODEL=sail/Sailor-7B-Chat

# Infer
python -m mlx_lm.generate \
    --model ${MODEL} \
    --eos-token "<|im_end|>" \
    --max-tokens 500 \
    --prompt "katopz คือใคร?"

# Train
python3 -m mlx_lm.lora \
    --model ${MODEL} \
    --train \
    --data /Users/katopz/git/mlx-examples/lora/data \
    --iters 300

# Test
python3 -m mlx_lm.lora \
    --model ${MODEL} \
    --adapter-file adapters.npz \
    --data /Users/katopz/git/mlx-examples/lora/data \
    --test

# Generate
python3 -m mlx_lm.generate \
    --model ${MODEL} \
    --adapter-file adapters.npz \
    --eos-token "<|im_end|>" \
    --max-tokens 500 \
    --prompt "katopz คือใคร?"

# Infer fused
python3 -m mlx_lm.generate \
    --model ./lora_fused_model \
    --eos-token "<|im_end|>" \
    --max-tokens 500 \
    --prompt "katopz คือใคร?"
```
