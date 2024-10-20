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
# MODEL=sail/Sailor-7B-Chat
MODEL=meta-llama/Llama-3.2-3B-Instruct

# Infer
python -m mlx_lm.generate \
    --model ${MODEL} \
    --eos-token "<|im_end|>" \
    --max-tokens 500 \
    --prompt "K Point คืออะไร ?"

# Train LoRA
python3 -m mlx_lm.lora \
    --model ${MODEL} \
    --train \
    --data /Users/katopz/git/mlx-examples/lora/data \
    --iters 500

# Train DoRA
python3 -m mlx_lm.lora \
    --model ${MODEL} \
    --train \
    --data /Users/katopz/git/mlx-examples/lora/data \
    --fine-tune-type dora
    --iters 500

# Test
python3 -m mlx_lm.lora \
    --model ${MODEL} \
    --adapter-path adapters \
    --data /Users/katopz/git/mlx-examples/lora/data \
    --test

# Generate
python3 -m mlx_lm.generate \
    --model ${MODEL} \
    --adapter-path adapters \
    --max-tokens 256 \
    --prompt "K Point คืออะไร ?"

# Infer fused
python3 -m mlx_lm.generate \
    --model ./lora_fused_model \
    # --eos-token "<|im_end|>" \
    --max-tokens 256 \
    --prompt "K Point คืออะไร ?"
```
