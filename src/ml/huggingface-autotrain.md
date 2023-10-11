## 🤗 HuggingFace - AutoTrain

### 📚 Knowledge Base

- `PEFT` [Parameter-Efficient Fine-Tuning](https://github.com/huggingface/peft)
- `TRL` [Transformer Reinforcement Learning](https://github.com/huggingface/trl)

> ref: [https://github.com/huggingface/trl/blob/main/examples/README.md]()

### ⚙️ Setup

```bash
# Install Python3 pip
sudo apt install python3-pip

# Install PEFT
pip install --upgrade bitsandbytes datasets accelerate loralib
pip install git+https://github.com/huggingface/peft.git

# Install TRL
pip install trl

# optional: wandb
pip install wandb

# Optional before Accelerate
pip3 install deepspeed

# Installing Accelerate
pip install accelerate
```

### 📝 Config

```bash
accelerate config
------------------------------------------------------------------------------------------------------------------------
In which compute environment are you running?
This machine
------------------------------------------------------------------------------------------------------------------------
Which type of machine are you using?
No distributed training
Do you want to run your training on CPU only (even if a GPU / Apple Silicon / Ascend NPU device is available)? [yes/NO]:
no
Do you wish to optimize your script with torch dynamo?[yes/NO]:no
Do you want to use DeepSpeed? [yes/NO]: yes
Do you want to specify a json file to a DeepSpeed config? [yes/NO]: no
------------------------------------------------------------------------------------------------------------------------
What should be your DeepSpeed's ZeRO optimization stage?
2
------------------------------------------------------------------------------------------------------------------------
Where to offload optimizer states?
cpu
------------------------------------------------------------------------------------------------------------------------
Where to offload parameters?
nvme
Nvme Path to offload parameters?
How many gradient accumulation steps you're passing in your script? [1]:
Do you want to use gradient clipping? [yes/NO]:
Do you want to enable `deepspeed.zero.Init` when using ZeRO Stage-3 for constructing massive models? [yes/NO]:
How many GPU(s) should be used for distributed training? [1]:
------------------------------------------------------------------------------------------------------------------------
Do you wish to use FP16 or BF16 (mixed precision)?
bf16
accelerate configuration saved at /home/katopz/.cache/huggingface/accelerate/default_config.yaml
katopz@shikuwa:~$ accelerate config
------------------------------------------------------------------------------------------------------------------------
In which compute environment are you running?
This machine
------------------------------------------------------------------------------------------------------------------------
Which type of machine are you using?
No distributed training
Do you want to run your training on CPU only (even if a GPU / Apple Silicon / Ascend NPU device is available)? [yes/NO]:
Do you wish to optimize your script with torch dynamo?[yes/NO]:yes
------------------------------------------------------------------------------------------------------------------------
Which dynamo backend would you like to use?
inductor
Do you want to customize the defaults sent to torch.compile? [yes/NO]:
Do you want to use DeepSpeed? [yes/NO]: yes
Do you want to specify a json file to a DeepSpeed config? [yes/NO]:
------------------------------------------------------------------------------------------------------------------------
What should be your DeepSpeed's ZeRO optimization stage?
2
------------------------------------------------------------------------------------------------------------------------
Where to offload optimizer states?
cpu
------------------------------------------------------------------------------------------------------------------------
Where to offload parameters?
nvme
Nvme Path to offload parameters?
How many gradient accumulation steps you're passing in your script? [1]:
Do you want to use gradient clipping? [yes/NO]: yes
What is the gradient clipping value? [1.0]:
Do you want to enable `deepspeed.zero.Init` when using ZeRO Stage-3 for constructing massive models? [yes/NO]: yes
How many GPU(s) should be used for distributed training? [1]:
------------------------------------------------------------------------------------------------------------------------
Do you wish to use FP16 or BF16 (mixed precision)?
bf16
accelerate configuration saved at /home/katopz/.cache/huggingface/accelerate/default_config.yaml
```

### 🪛 Fine tune

![](/assets/kat.png) <span class="speech-bubble">Let's fine-tune [Mistral](https://mistral.ai/) locally.</span>

```bash
# Setup
pip install -U autotrain-advanced
autotrain setup
autotrain llm --help

# Fine tune
autotrain llm \
  --train \
  --model mistralai/Mistral-7B-Instruct-v0.1 \
  --data-path $DATA_PATH \
  --use-peft\
  --use-int4\
  --lr 2e-4 \
  --batch-size 4 \
  --epochs 1 \
  --trainer sft \
  --username $USER_NAME \
  --token $HF_TOKEN \
  --project-name $PROJECT_NAME \
  --target-modules q_proj,v_proj \
  --push-to-hub
```
