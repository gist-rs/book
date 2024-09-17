# FLUX.1

- model: [https://huggingface.co/black-forest-labs/FLUX.1-dev]()
- mflux (mlx): [https://github.com/filipstrand/mflux]()

## dev

> slower

```
mflux-generate --model dev --prompt "Ragdoll cats typing on macbook and look back to camera" --steps 10 --seed 2 -q 8
```

> 2.72s user 11.48s system 12% cpu 1:57.52 total

![](/ml/assets/flux.1-dev.png)

## schnell

> faster

```
mflux-generate --model schnell --prompt "Ragdoll cats typing on macbook and look back to camera" --steps 2 --seed 2 -q 8
```

> 2.02s user 9.37s system 38% cpu 29.611 total

![](/ml/assets/flux.1-schnell.png)
