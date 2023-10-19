# LLM Train and FineTune

- [x] Try [Colab Notebook to Finetuning Mistral-7b-Instruct](https://adithyask.medium.com/a-beginners-guide-to-fine-tuning-mistral-7b-instruct-model-0f39647b20fe): https://github.com/adithya-s-k/CompanionLLM

The notebook needs to add `pad_token_id=2` when calling merged_model.generate() in Test the merged model:

```python
outputs = merged_model.generate(input_ids=input_ids, pad_token_id=2,
max_new_tokens=100, do_sample=True, top_p=0.9,temperature=0.5)
```

Fixed notebook is [Mistral_7B_qLora_Finetuning.ipynb](./Mistral_7B_qLora_Finetuning.ipynb). Prompt formatting is still questioned.

## TODO

- [ ] Read, try and summary: https://github.com/replit/replitLM#alpaca-style-instruct-tuning-with-hugging-face-transformers
- [ ] Try fine-tune-mistral: https://github.com/abacaj/fine-tune-mistral
- [ ] Try LocalGPT: https://github.com/PromtEngineer/localGPT
- [ ] Read and summary: https://twitter.com/manelferreira_/status/1711788177458090388
- [ ] Read and summary: https://docs.llamaindex.ai/en/stable/core_modules/data_modules/index/metadata_extraction.html
- [ ] LLaVA: Visual and language AI with GPT-4, instruction tuning to visual data.
- [ ] MUFFIN: follow Scaling Tasks per Input paradigm to collect multiple task instructions for a single input. (Keywords: Instruction Brainstorm, Instruction Rematching)
