# Dia

> [How to setup and run `Dia-1.6B` on mac silicon](voices/dia.md)

> 🚧 Currently it has a lot of bug, so be patient.

- [https://github.com/nari-labs/dia]()
- [https://huggingface.co/spaces/nari-labs/Dia-1.6B]()
- [https://github.com/Blaizzy/mlx-audio]()
- [https://huggingface.co/mlx-community/Dia-1.6B]()

```
# Setup
brew install python@3.10
python3.10 -m venv .venv
source .venv/bin/activate
python --version
pip install --upgrade pip

## Install from released
pip install -U mlx-audio

## Or from source
pip install git+https://github.com/Blaizzy/mlx-audio
pip install -r requirements.txt

## Cross check version
pip list | grep mlx

# Run
mlx_audio.tts.generate --text "Wow. Amazing." --play

mlx_audio.tts.generate --model mlx-community/Dia-1.6B --text "[S1] Wow. Amazing. (laughs)" --play

mlx_audio.tts.generate --model mlx-community/Dia-1.6B --text "[S1] Dia is an open weights text to dialogue model. [S2] You get full control over scripts and voices. [S1] Wow. Amazing. (laughs) [S2] Try it now on Git hub or Hugging Face." --play

# Ref
mlx_audio.tts.generate --model mlx-community/Dia-1.6B --text "[S1]Hello, world(laugh)" --play --ref_audio ./conversational_a.wav
```
