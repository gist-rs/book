# LLaMA-rs

**LLaMA-rs** is a Rust port of the
[llama.cpp](https://github.com/ggerganov/llama.cpp) project. This allows running
inference for Facebook's [LLaMA](https://github.com/facebookresearch/llama)
model on a CPU with good performance using full precision, f16 or 4-bit
quantized versions of the model.

Just like its C++ counterpart, it is powered by the
[`ggml`](https://github.com/ggerganov/ggml) tensor library, achieving the same performance as the original code.

## Source

- https://github.com/setzer22/llama-rs

<details>
<summary>Conversions note</summary>

```mermaid
graph TD;
  A("PyTorch") --"<pre>1️⃣/2️⃣&nbsp;export_state_dict_checkpoint.py</pre>PyTorch model checkpoints (pth)"--> B(Python) --"<pre>3️⃣&nbsp;convert-pth-to-ggml.py</pre>Geometric Deep Learning Markup Language (ggml)"--> C(C++)--"<pre>4️⃣&nbsp;quantize.cpp</pre>Quantized ggml (bin)"-->D(Rust);
```

1️⃣ [tloen/alpaca-lora/export_state_dict_checkpoint.py](https://github.com/tloen/alpaca-lora/blob/main/export_state_dict_checkpoint.py) (llama-7b-hf)  
2️⃣ [jankais3r/LLaMA_MPS/export_state_dict_checkpoint.py](https://github.com/jankais3r/LLaMA_MPS/blob/main/export_state_dict_checkpoint.py) (llama-13b-hf)  
3️⃣ [llama.cpp/convert-pth-to-ggml.py](https://github.com/ggerganov/llama.cpp/blob/master/convert-pth-to-ggml.py)  
4️⃣ [llama.cpp/quantize.cpp](https://github.com/ggerganov/llama.cpp/blob/master/quantize.cpp)

</details>

---

## Overview

```mermaid
graph LR;
A --Apple Silicon GPU--> AA("🐍 LLaMA_MPS")
A -..-> AAA("Raspberry Pi, Pixel 5, iPhone, NodeJS")
A("🐍 llama") --"4-bit"--> B("🐇 llama")
B --port w/ ggml--> C("🦀 llama-rs")
A --"16,32-bit"--> CC("🦀 RLLaMA")
C --"napi-rs"--> I("🐥 llama-node")
E --"fine-tuning to obey ix"--> D("🐇 alpaca")
A --instruction-following--> E("🐍 alpaca") --LoRa--> F("🐍 alpaca-lora")
E --instruction-following--> H("🐍 codealpaca")
B --BLOOM-like--> BB("🐇 bloomz")
BB --LoRA--> DDDD("🐍 BLOOM-LoRA")
D --"fine-tunes the GPT-J 6B"--> DD("🐍 Dolly")
D --"instruction-tuned Flan-T5"--> DDD("🐍 Flan-Alpaca")
D --Alpaca_data_cleaned.json--> DDDD
E --RNN-->EE("🐍 RWKV-LM")
H --finetuned--> EE
EE("🐍 RWKV-LM") -..-> EEE("🦀 smolrsrwkv")
A --"GPT-3.5-Turbo"--> FF("🐍 gpt4all-lora")
A --"Apache0/nanoGPT"--> AAAA("🐍 Lit-LLaMA")
```

- [🐍 llama](https://github.com/facebookresearch/llama): Open and Efficient Foundation Language Models.
- [🐍 LLaMA_MPS](https://github.com/jankais3r/LLaMA_MPS): Run LLaMA (and Stanford-Alpaca) inference on Apple Silicon GPUs.
- [🐇 llama](https://github.com/ggerganov/llama.cpp): Inference of LLaMA model in pure C/C++.
- [🐇 alpaca](https://github.com/antimatter15/alpaca.cpp): This combines the LLaMA foundation model with an open reproduction of Stanford Alpaca a fine-tuning of the base model to obey instructions (akin to the RLHF used to train ChatGPT) and a set of modifications to llama.cpp to add a chat interface.
- [🦀 llama-rs](https://github.com/setzer22/llama-rs): Do the LLaMA thing, but now in Rust 🦀🚀🦙
- [🐍 alpaca](https://github.com/tatsu-lab/stanford_alpaca): Stanford Alpaca: An Instruction-following LLaMA Model
- [🐍 codealpaca](https://github.com/sahil280114/codealpaca): An Instruction-following LLaMA Model trained on code generation instructions.
- [🐍 alpaca-lora](https://github.com/tloen/alpaca-lora): Low-Rank LLaMA Instruct-Tuning
- [🐥 llama-node](https://github.com/hlhr202/llama-node): nodejs client library for llama LLM built on top of llama-rs. It uses napi-rs as nodejs and native communications.
- [🦀 RLLaMA](https://github.com/Noeda/rllama): Rust+OpenCL+AVX2 implementation of LLaMA inference code.
- [🐍 Dolly](https://github.com/databrickslabs/dolly): This fine-tunes the GPT-J 6B model on the Alpaca dataset using a Databricks notebook.
- [🐍 Flan-Alpaca](https://github.com/declare-lab/flan-alpaca): Instruction Tuning from Humans and Machines.
- [🐇 bloomz](https://github.com/NouamaneTazi/bloomz.cpp): Inference of HuggingFace's BLOOM-like models in pure C/C++ built on top of the amazing llama.cpp.
- [🐍 BLOOM-LoRA](https://github.com/linhduongtuan/BLOOM-LORA): Low-Rank LLaMA Instruct-Tuning.
- [🐍 RWKV-LM](https://github.com/BlinkDL/RWKV-LM): [🤗](https://huggingface.co/spaces/BlinkDL/Raven-RWKV-7B) RWKV is an RNN with transformer-level LLM performance. It can be directly trained like a GPT (parallelizable). So it's combining the best of RNN and transformer - great performance, fast inference, saves VRAM, fast training, "infinite" ctx_len, and free sentence embedding.
- [🦀 smolrsrwkv](https://github.com/KerfuffleV2/smolrsrwkv): A very basic example of the RWKV approach to language models written in Rust by someone that knows basically nothing about math or neural networks.
- [🐍 gpt4all-lora](https://github.com/nomic-ai/gpt4all): A chatbot trained on a massive collection of clean assistant data including code, stories and dialogue.
- [🐍 Lit-LLaMA](https://github.com/Lightning-AI/lit-llama): Independent implementation of LLaMA that is fully open source under the Apache 2.0 license. This implementation builds on nanoGPT.

## Alternatives

![](./assets/scaling-laws-blog-comparison.png)

```mermaid
graph LR;
Z("GPT")
Z-.Chinchilla formula..->Y("Cerebras-GPT")
Z-.Flamingo-style LMMs..-X("OpenFlamingo")
```

- [🐍 Cerebras-GPT](https://www.cerebras.net/blog/cerebras-gpt-a-family-of-open-compute-efficient-large-language-models/): a family of seven GPT models ranging from 111 million to 13 billion parameters. Trained using the Chinchilla formula, these models provide the highest accuracy for a given compute budget. Cerebras-GPT has faster training times, lower training costs, and consumes less energy than any publicly available model to date.
- [🐍 OpenFlamingo](https://laion.ai/blog/open-flamingo/): a framework that enables training and evaluation of large multimodal models (LMMs).

## Tools

```mermaid
graph TD;
AAA("🐍 llama") --> BB("🐍 LLaMA-Adapter")
AAA --> A
A("🐍 langchain")
A --port--> AA("🐥 langchainjs")
AA --> B("🐥 langchain-alpaca")
D("🐇 alpaca") --> B
E-..-D
E("🐇 llama") --ggml/13B--> H
F("🐇 whisper") --whisper-small--> H
H("🐇 talk")
I("🐍 chatgpt-retrieval-plugin") --> II("🐍 llama-retrieval-plugin")
```

- [🐍 langchain](https://github.com/hwchase17/langchain): Building applications with LLMs through composability.
- [🐥 langchainjs](https://github.com/hwchase17/langchainjs): langchain in js.
- [🐥 langchain-alpaca](https://github.com/linonetwo/langchain-alpaca): Run alpaca LLM fully locally in langchain.
- [🐇 whisper](https://github.com/ggerganov/whisper.cpp): High-performance inference of OpenAI's Whisper automatic speech recognition (ASR) model.
- [🤗 whisper-small](https://huggingface.co/openai/whisper-small): Whisper is a pre-trained model for automatic speech recognition (ASR) and speech translation. Trained on 680k hours of labelled data, Whisper models demonstrate a strong ability to generalise to many datasets and domains without the need for fine-tuning.
- [🐇 talk](https://github.com/ggerganov/whisper.cpp/tree/master/examples/talk): Talk with an Artificial Intelligence in your terminal.
- [chatgpt-retrieval-plugin](https://github.com/openai/chatgpt-retrieval-plugin): The ChatGPT Retrieval Plugin lets you easily search and find personal or work documents by asking questions in everyday language.
- [llama-retrieval-plugin](https://github.com/lastmile-ai/llama-retrieval-plugin): LLaMa retrieval plugin script using OpenAI's retrieval plugin
- [🐍 LLaMA-Adapter](https://github.com/ZrrSkywalker/LLaMA-Adapter): LLaMA-Adapter: Efficient Fine-tuning of Language Models with Zero-init Attention. Using 52K self-instruct demonstrations, LLaMA-Adapter only introduces 1.2M learnable parameters upon the frozen LLaMA 7B model, and costs less than one hour for fine-tuning on 8 A100 GPUs .

## Hugging Face 🤗

- [🤗 Code Alpaca](https://huggingface.co/spaces/sahil2801/CodeAlpaca): `13B`, The Code Alpaca models are fine-tuned from a 7B and 13B LLaMA model on 20K instruction-following data generated by the techniques in the Self-Instruct [1] paper, with some modifications that we discuss in the next section. Evals are still a todo.
- [🤗 Alpaca-LoRA-Serve](https://huggingface.co/spaces/chansung/Alpaca-LoRA-Serve): `7B`, Instruction fine-tuned version of LLaMA from Meta AI. Alpaca-LoRA is Low-Rank LLaMA Instruct-Tuning which is inspired by Stanford Alpaca project. This demo application currently runs 7B version on a T4 instance.

## ETC

> Refer to: https://replicate.com/blog/llama-roundup

- [Running LLaMA on a Raspberry Pi](https://twitter.com/miolini/status/1634982361757790209) by Artem Andreenko.
- [Running LLaMA on a Pixel 5](https://twitter.com/ggerganov/status/1635605532726681600) by Georgi Gerganov.
- [Run LLaMA and Alpaca with a one-liner](https://cocktailpeanut.github.io/dalai) – `npx dalai llama`
- [Train and run Stanford Alpaca on your own machine](https://replicate.com/blog/replicate-alpaca) from replicate.
- [Fine-tune LLaMA to speak like Homer Simpson](https://replicate.com/blog/fine-tune-llama-to-speak-like-homer-simpson) from replicate.
- [Llamero](https://github.com/mpociot/llamero/) – A GUI application to easily try out Facebook's LLaMA models by Marcel Pociot.
