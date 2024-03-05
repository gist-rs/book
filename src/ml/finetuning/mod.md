# LLM FineTuning

## Raw

- [RLHF in 2024 with DPO & Hugging Face](https://www.philschmid.de/dpo-align-llms-in-2024-with-trl)
- [Direct Preference Optimization (DPO): A Simplified Approach to Fine-tuning Large Language Models](https://ai.plainenglish.io/direct-preference-optimization-dpo-a-simplified-approach-to-fine-tuning-large-language-models-bae1c6d7ec29)
- [Fine-tuning SeaLLM on Your Own Dataset with QLoRA on RTX4090](./finetune-seallm.ipynb)
- [MLX: Quantize, LoRA, QLoRA, Fuse](./mlx.md)

## Direct Preference Optimization (DPO)

### Steps

```mermaid
graph LR
  A["1️⃣ Supervised Fine-tuning (SFT)"]--->B["2️⃣ Preference Learning"]
```

### Datasets

> train.json, test.json

```json
[
  { "role": "system", "content": "You are..." },
  { "role": "user", "content": "..." },
  { "role": "assistant", "content": "..." }
]
```

## Brain dump (WIP)

### A Beginner’s Guide to Fine-Tuning Mistral 7B Instruct Model

> Fixed notebook is [Mistral_7B_qLora_Finetuning.ipynb](./Mistral_7B_qLora_Finetuning.ipynb). But prompt formatting is still in doubt.

- Colab: https://adithyask.medium.com/a-beginners-guide-to-fine-tuning-mistral-7b-instruct-model-0f39647b20fe
- Source: https://github.com/adithya-s-k/CompanionLLM
- ⚠️ This notebook needs to add `pad_token_id=2` when calling merged_model.generate() in Test the merged model:

  ```python
  outputs = merged_model.generate(input_ids=input_ids, pad_token_id=2,
  max_new_tokens=100, do_sample=True, top_p=0.9,temperature=0.5)
  ```
