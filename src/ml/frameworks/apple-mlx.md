# Apple - MLX

> https://github.com/ml-explore  
> https://github.com/ml-explore/mlx-examples

## LoRA example

> https://github.com/ml-explore/mlx-examples/tree/main/lora

```
# Setup
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt

# Download model
curl -O https://files.mistral-7b-v0-1.mistral.ai/mistral-7B-v0.1.tar
tar -xf mistral-7B-v0.1.tar

# Convert
python convert.py \
    --torch-model mistral-7B-v0.1 \
    --mlx-model mlx_mistral_7b.npz

# Fine-tune for ./adapters.npz
python lora.py --model mlx_mistral_7b \
               --train \
               --iters 600

# Eval with ./adapters.npz
python lora.py --model mlx_mistral_7b \
               --adapter-file ./adapters.npz \
               --test
# Generate
python lora.py --model mlx_mistral_7b \
               --adapter-file ./adapters.npz \
               --num-tokens 50 \
               --prompt "table: 1-10015132-16
columns: Player, No., Nationality, Position, Years in Toronto, School/Club Team
Q: What is terrence ross' nationality
A: "

# Fine-tune from custom data jsonl
# --batch-size 4, 2, 1
# --lora-layers 16, 8, 4
python lora.py \
   --model mlx_mistral_7b \
   --train \
   --batch-size 2 \
   --lora-layers 16

# Test jsonl
python lora.py --model mlx_mistral_7b \
               --adapter-file ./adapters.npz \
               --num-tokens 50 \
               --prompt "table: 1-10015132-11
columns: Player, No., Nationality, Position, Years in Toronto, School/Club Team
Q: What position does the player who played for butler cc (ks) play?
A: "
```
