# Fish Agent V0.1 3B

> How to setup and run `[fishaudio/fish-agent-v0.1-3b](https://huggingface.co/fishaudio/fish-agent-v0.1-3b)` on mac silicon.

## Run `Fish Agent` on Mac M3

```bash
brew install portaudio
brew install python@3.10
python3.10 -m venv .venv
source .venv/bin/activate
python --version
pip install --upgrade pip
pip install torch==2.4.1 torchvision==0.19.1 torchaudio==2.4.1
pip install -e '.[stable]'
pip install cachetools
huggingface-cli download fishaudio/fish-speech-1.4 --local-dir checkpoints/fish-speech-1.4
python tools/webui.py --device mps
```
