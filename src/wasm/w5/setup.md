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
wasm-pack new hello-wasm              # 👈 Create new app.

wasm-pack test --headless --firefox   # 👈 Test with headless firefox.

wasm-pack build                       # 👈 Build to pkg dir.

wasm-pack publish                     # 👈 Publish to npm.
```

![](/assets/kat.png) We can use with `cargo watch` like this 👇 to watch with ignore(`-i`) and auto refresh the tests.

```shell
cargo watch -i .gitignore -i "pkg/*" -s "wasm-pack test --headless --firefox"
```

## 3️⃣ Hello World

// 🚧 TODO : compile https://rustwasm.github.io/docs/wasm-pack/quickstart.html
