## Stable Diffusion

```mermaid
graph LR;
Text(Text) -- prompt --> txt2img([txt2img]) --> Image
Image -.-> img2img([img2img])

ControlNet(ControlNet) --> upscale["Upscale:tile"] --> Image
ControlNet --> shuffle --> Image
ControlNet --> inpaint --> Image
ControlNet --> seg["Semantic Segmentation"] --> Image
ControlNet --> depth,normalbae,openpose --> Image
ControlNet --> mlsd,canny,softedge,scribble,lineart --> Image
ControlNet --> ip2p["Instruct Pix2Pix (ip2p)"] --> Image

img2img -- "Fix small" --> HiRes-Image
img2img -- "Fix color" --> VAE(["Variational Auto-Encoder (VAE)"])

VAE -- "smoother" --> MSE
VAE -- "original" --> EMA
```

## Models

```mermaid
graph LR;

EasyNegative["<a href='https://huggingface.co/datasets/gsdf/EasyNegative'>EasyNegative</a><br/>(Negative Embedding)"] .- counterfeit

defacta3th[<img class='thumb128' src='https://imagecache.civitai.com/xG1nkqKTMzGDvpLrqFT7WA/83eee1eb-fd3a-4497-b04a-64da85667d00/width=450/00256-4261342171.jpeg' style='object-fit:cover'/><br/><a href='https://civitai.com/models/45804/defacta3th'>Defacta3th</a>] --> defacounter-mix[<img class='thumb128' src='https://imagecache.civitai.com/xG1nkqKTMzGDvpLrqFT7WA/27c1e448-198b-46b9-9869-9081e1883400/width=450/00216-1523285327.jpeg' style='object-fit:cover'/><br/> <a href='https://civitai.com/models/55237'> defacounter-mix</a>]
counterfeit[<img class='thumb128' src='https://imagecache.civitai.com/xG1nkqKTMzGDvpLrqFT7WA/5f06c30d-0169-4d58-381c-3ee00ea90100/width=450/002.jpeg' style='object-fit:cover'/><br/> <a href='https://civitai.com/models/4468/counterfeit-v30'>Counterfeit-V3.0</a>] --> defacounter-mix
```

## Library

- [tch-rs](https://github.com/LaurentMazare/tch-rs): Rust bindings for the C++ api of PyTorch. The goal of the tch crate is to provide some thin wrappers around the C++ PyTorch api (a.k.a. libtorch). It aims at staying as close as possible to the original C++ api.
- [tch-m1](https://github.com/ssoudan/tch-m1): how to use LaurentMazare/tch-rs on M1.
- [burn-rs](https://burn-rs.github.io/): This library strives to serve as a comprehensive deep learning framework, offering exceptional flexibility and written in Rust. Our objective is to cater to both researchers and practitioners by simplifying the process of experimenting, training, and deploying models.
- [diffusers-rs](https://github.com/LaurentMazare/diffusers-rs): An implementation of the diffusers api in Rust.

## 3D

- [ReMoDiffuse](https://mingyuan-zhang.github.io/projects/ReMoDiffuse.html): ReMoDiffuse is a retrieval-augmented 3D human motion diffusion model. Benefiting from the extra knowledge from the retrieved samples, ReMoDiffuse is able to achieve high-fidelity on the given prompts.

## ControlNet

- [ControlNetMediaPipeFace](https://huggingface.co/spaces/CrucibleAI/ControlNetMediaPipeFaceSD21)

## Blender

- [Dream Texture](https://github.com/carson-katri/dream-textures): Stable Diffusion built-in to Blender.

## 2D

- [Graphite](https://github.com/GraphiteEditor/Graphite): Redefining state-of-the-art graphics editing + stable diffusion.
