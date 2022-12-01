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

![](/assets/kat.png) We did not start with default `alert` example because it's too boring, Let's start with `sync` and `async` instead.

#### `Cargo.toml`

```toml
{{#include ../../../examples/w5/hello-wasm/Cargo.toml}}
```

#### `utils.rs`

```rust,no_run
{{#include ../../../examples/w5/hello-wasm/src/utils.rs}}
```

#### `lib.rs`

```rust,no_run
{{#include ../../../examples/w5/hello-wasm/src/lib.rs}}
```

#### `tests/web.rs`

```rust,no_run
{{#include ../../../examples/w5/hello-wasm/tests/web.rs}}
```
