# Apple - MLX

- https://github.com/ml-explore
- https://github.com/ml-explore/mlx-examples

## Infer (huggingface)

```bash
# Setup
python3 -m venv .venv
source .venv/bin/activate
pip install mlx-lm

# Infer (multi-lang)
MODEL=SeaLLMs/SeaLLM-7B-v2
MODEL_NAME=SeaLLM-7B-v2
# MODEL=aisingapore/sealion7b-instruct-nc
# MODEL_NAME=sealion7b-instruct-nc

python -m mlx_lm.generate --model ${MODEL} --prompt "สวัสดี"
python -m mlx_lm.convert --hf-path ${MODEL} -q --upload-repo mlx-community/${MODEL_NAME}-4bit-mlx
python -m mlx_lm.generate --model mlx-community/SeaLLM-7B-v2-4bit-mlx --prompt "สอนเขียน helloworld ด้วย Rust หน่อย"
```

## Infer (manual)

```bash
# Setup
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt

# Download model (outdate, not recommend)
curl -LO https://files.mistral-7b-v0-1.mistral.ai/mistral-7B-v0.1.tar
tar -xf mistral-7B-v0.1.tar

# Convert to 4bits quantize
python convert.py \
    --torch-path mistral-7B-v0.1 -q

# Or just Download npz model
curl -LO https://huggingface.co/mlx-community/Mistral-7B-Instruct-v0.2/resolve/main/config.json
curl -LO https://huggingface.co/mlx-community/Mistral-7B-Instruct-v0.2/resolve/main/tokenizer.model
curl -LO https://huggingface.co/mlx-community/Mistral-7B-Instruct-v0.2/resolve/main/weights.npz

# Infer
python mistral.py --prompt "Write helloworld code in Rust"
```

## LoRA example

> https://github.com/ml-explore/mlx-examples/tree/main/lora

```bash
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

## Convert

```bash
# Get access token
open https://huggingface.co/settings/tokens

# Install huggingface cli
pip install --upgrade huggingface_hub
huggingface-cli login

# Setup
git clone https://github.com/ml-explore/mlx-examples
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt

# convert and upload
python3 -m mlx_lm.convert --hf-path mlabonne/NeuralBeagle14-7B --upload-repo mlx-community/NeuralBeagle14-7B-mlx
python3 -m mlx_lm.convert -q --hf-path mlabonne/NeuralBeagle14-7B --upload-repo mlx-community/NeuralBeagle14-7B-4bit-mlx
```

## Use with `mlx_lm` cli

```bash
pip install -U mlx-lm
python3 -m mlx_lm.generate --model mlx-community/NeuralBeagle14-7B --prompt "<|im_start|>system\nYou are the best programmer<|im_end|>\n<|im_start|>user\nWrite helloworld code in Rust.<|im_end|>\n<|im_start|>assistant\n" --max-tokens 2048
```
