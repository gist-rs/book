### MiniCPM

## Handy

```
python -m torch.utils.collect_env

pip install deepspeed
ds_report
```

---

## WSL2

### Issue 1

#### `AttributeError: 'DeepSpeedCPUAdam' object has no attribute 'ds_opt_adam'`

> ref: https://github.com/microsoft/DeepSpeed/issues/1846

#### Attempt (didn't work)

```
DS_BUILD_CPU_ADAM=1 BUILD_UTILS=1 pip install deepspeed -U
```

---

## MPS

### Issue 1

#### `RuntimeError: Distributed package doesn't have NCCL built in`

#### Attempt (not related)

> https://ryandam.net/blog/2023/08/03/using-llama.cpp/index.html
