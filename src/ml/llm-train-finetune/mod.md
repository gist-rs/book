# LLM Train and FineTune

## Overview

![](./llm-langchain-flow.png)

> ref: https://github.com/chatchat-space/Langchain-Chatchat/blob/master/README_en.md

## Brain dump (WIP)

### A Beginner’s Guide to Fine-Tuning Mistral 7B Instruct Model

> Fixed notebook is [Mistral_7B_qLora_Finetuning.ipynb](./Mistral_7B_qLora_Finetuning.ipynb). But prompt formatting is still questioned.

- Colab: [https://adithyask.medium.com/a-beginners-guide-to-fine-tuning-mistral-7b-instruct-model-0f39647b20fe]()
- Source: [https://github.com/adithya-s-k/CompanionLLM]()
- ⚠️ This notebook needs to add `pad_token_id=2` when calling merged_model.generate() in Test the merged model:

  ```python
  outputs = merged_model.generate(input_ids=input_ids, pad_token_id=2,
  max_new_tokens=100, do_sample=True, top_p=0.9,temperature=0.5)
  ```

### Mistral-7B-Instruct Multiple-PDF Chatbot with Langchain & Streamlit

> Fixed notebook is [Chat_with_MultiplePDFs_Mistral_7B_Instruct1.ipynb](./Chat_with_MultiplePDFs_Mistral_7B_Instruct1.ipynb)

- Youtube: [https://www.youtube.com/watch?v=tqpXvPzteT4]()
- Colab: [https://colab.research.google.com/drive/11sf5LAF5EC1M0cDh-pUyowvS7EflwlMH?usp=sharing]()
- ⚠️ This notebook didn't use GPU. To support GPU will need.
  ```bash
  %%bash
  CMAKE_ARGS="-DLLAMA_CUBLAS=on" FORCE_CMAKE=1 pip install llama-cpp-python
  ```
  and
  ```python
  llm = LlamaCpp(
    streaming = True,
    model_path="./mistral-7b-instruct-v0.1.Q4_K_M.gguf",
    temperature=0.75,
    top_p=1,
    verbose=True,
    n_ctx=4096,
    n_gpu_layers=30,
    n_threads=2,
    n_batch=521,
  )
  ```

## TODO

- [x] Understand pgvector: https://bugbytes.io/posts/vector-databases-pgvector-and-langchain/
- [ ] Try RAG: https://bugbytes.io/posts/retrieval-augmented-generation-with-langchain-and-pgvector/
- [ ] Create POC PDF pipeline service.

## TOREAD

- [ ] Read, try and summary: https://github.com/replit/replitLM#alpaca-style-instruct-tuning-with-hugging-face-transformers
- [ ] Try fine-tune-mistral: https://github.com/abacaj/fine-tune-mistral
- [ ] Try fine-tune-mistral: https://github.com/brevdev/notebooks/blob/main/mistral-finetune-own-data.ipynb
- [ ] Try LocalGPT: https://github.com/PromtEngineer/localGPT
- [ ] Read and summary: https://twitter.com/manelferreira_/status/1711788177458090388
- [ ] Read and summary: https://docs.llamaindex.ai/en/stable/core_modules/data_modules/index/metadata_extraction.html
- [ ] LLaVA: Visual and language AI with GPT-4, instruction tuning to visual data.
- [ ] MUFFIN: follow Scaling Tasks per Input paradigm to collect multiple task instructions for a single input. (Keywords: Instruction Brainstorm, Instruction Rematching)
- [ ] Take a look: https://github.com/chatchat-space/Langchain-Chatchat
