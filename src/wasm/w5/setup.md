# Setup

## 1️⃣ tools

1. Install [rustup](https://rustup.rs/)

   ```shell
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

1. Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
   ```shell
   curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
   ```

## 2️⃣ Use `wasmpack`

```shell
wasm-pack new hello-wasm              # 👈 create new app.

wasm-pack test --headless --firefox   # 👈 test with headless firefox.

wasm-pack build                       # 👈 build to pkg dir.

wasm-pack publish                     # 👈 publish to npm.
```

## 3️⃣ Hello World

// 🚧 TODO : compile https://rustwasm.github.io/docs/wasm-pack/quickstart.html
