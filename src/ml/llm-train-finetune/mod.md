# LLM Train and FineTune

## Overview

![](./llm-langchain-flow.png)

> ref: https://github.com/chatchat-space/Langchain-Chatchat/blob/master/README_en.md

## Pipelines

> Ref: https://ai.plainenglish.io/direct-preference-optimization-dpo-a-simplified-approach-to-fine-tuning-large-language-models-bae1c6d7ec29

```mermaid
graph LR
  A["Direct Preference Optimization (DPO)"]---B["1️⃣ Supervised Fine-tuning (SFT)"]
  A---C["2️⃣ Preference Learning"]
```

## Brain dump (WIP)

### A Beginner’s Guide to Fine-Tuning Mistral 7B Instruct Model

> Fixed notebook is [Mistral_7B_qLora_Finetuning.ipynb](./Mistral_7B_qLora_Finetuning.ipynb). But prompt formatting is still in doubt.

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
  !CMAKE_ARGS="-DLLAMA_CUBLAS=on" FORCE_CMAKE=1 pip install llama-cpp-python
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

- [x] Understand `pgvector`: https://bugbytes.io/posts/vector-databases-pgvector-and-langchain/
- [ ] `RAG` with `Qdrant`: https://github.com/qdrant/rust-client

## TOTRY - `TimeGPT`

- [ ] Read `TimeGPT`: https://docs.nixtla.io/docs
- [ ] Try `TimeGPT` with [LlamaIndex's Supabase Vector Store](https://colab.research.google.com/github/supabase/supabase/blob/master/examples/ai/llamaindex/llamaindex.ipynb).

## TOTRY - `ChatWithPDF`

- [ ] Use `MistralLite` for longer context retrieval: https://huggingface.co/amazon/MistralLite
- [ ] try [Using MistralLite with HuggingFace Transformers and FlashAttention-2](https://github.com/awslabs/extending-the-context-length-of-open-source-llms/blob/main/MistralLite/huggingface-transformers/example_usage.ipynb)
- [ ] Try: https://github.com/aigeek0x0/zephyr-7b-alpha-langchain-chatbot
- [ ] Take a look: https://github.com/chatchat-space/Langchain-Chatchat
- [ ] Try `RAG` with `pgvector` for faster query: https://bugbytes.io/posts/retrieval-augmented-generation-with-langchain-and-pgvector/

## TOREAD

- [ ] Optimizing Text Embeddings with HuggingFace’s text-embeddings-inference Server and `LlamaIndex`: https://levelup.gitconnected.com/optimizing-text-embeddings-with-huggingfaces-text-embeddings-inference-server-and-llamaindex-ef7df35882a4
- [ ] Demystifying Advanced `RAG` Pipelines: https://github.com/pchunduri6/rag-demystified
- [ ] Jina 8K Context Window `Embeddings`: https://github.com/run-llama/llama_index/blob/main/docs/examples/embeddings/jina_embeddings.ipynb
- [ ] Read, try and summary: https://github.com/replit/replitLM#alpaca-style-instruct-tuning-with-hugging-face-transformers
- [ ] Try fine-tune-mistral: https://github.com/abacaj/fine-tune-mistral
- [ ] Try fine-tune-mistral: https://github.com/brevdev/notebooks/blob/main/mistral-finetune-own-data.ipynb
- [ ] Try `LocalGPT`: https://github.com/PromtEngineer/localGPT
- [ ] Read and summary: https://twitter.com/manelferreira_/status/1711788177458090388
- [ ] Read and summary: https://docs.llamaindex.ai/en/stable/core_modules/data_modules/index/metadata_extraction.html
- [ ] `LLaVA`: Visual and language AI with GPT-4, instruction tuning to visual data.
- [ ] `MUFFIN`: follow Scaling Tasks per Input paradigm to collect multiple task instructions for a single input. (Keywords: Instruction Brainstorm, Instruction Rematching)
