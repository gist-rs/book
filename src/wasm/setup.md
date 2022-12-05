# Setup

## 1️⃣ tools

1. Install [rustup](https://rustup.rs/)

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

1. Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
   ```bash
   curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
   ```

## 2️⃣ Use `wasmpack`

```bash
wasm-pack new hello-wasm              # 👈 Create new app.

wasm-pack test --headless --firefox   # 👈 Test with headless firefox.

wasm-pack build --target web          # 👈 Build (target web) to pkg dir.

wasm-pack publish                     # 👈 Publish to npm.
```

![](/assets/kat.png) We can use with `cargo watch` like this 👇 to watch with ignore(`-i`) and auto refresh the tests.

```bash
cargo watch -i .gitignore -i "pkg/*" -s "wasm-pack test --headless --firefox"
```

## 3️⃣ Hello World

```yml
🗂 hello-wasm
│
├─ 📂 pkg
│  ├─ 📄 hello_wasm_bg.wasm         # 👈 bg = bindgen.
│  ├─ 📄 hello_wasm_bg.wasm.d.ts    # 👈 for TypeScript.
│  ├─ 📄 hello_wasm.d.ts            # 👈 for TypeScript.
│  ├─ 📄 hello_wasm.js              # 👈 JavaScript.
│  └─ 📄 package.json               # 👈 For load as module.
│
├─ 📂 src
│  ├─ 📄 lib.rs     # 👈 lib entrypoint.
│  └─ 📄 utils.ra   # 👈 some utils.
│
├─ 📂 tests
│  └─ 📄 web.rs     # 👈 test file via web.
│
└─ 📦 Cargo.toml
```

└─ 📦 Cargo.toml

```yaml
[package]
name = "hello-wasm"
version = "0.1.0"
authors = ["katopz <katopz@gmail.com>"]
edition = "2018"

[lib]
crate-type = ["cdylib", "rlib"]

[features]
default = ["console_error_panic_hook"]

[dependencies]
wasm-bindgen = "0.2.63"

# The `console_error_panic_hook` crate provides better debugging of panics by
# logging them with `console.error`. This is great for development, but requires
# all the `std::fmt` and `std::panicking` infrastructure, so isn't great for
# code size when deploying.
console_error_panic_hook = { version = "0.1.6", optional = true }

# `wee_alloc` is a tiny allocator for wasm that is only ~1K in code size
# compared to the default allocator's ~10K. It is slower than the default
# allocator, however.
wee_alloc = { version = "0.4.5", optional = true }

[dev-dependencies]
wasm-bindgen-test = "0.3.13"

[profile.release]
# Tell `rustc` to optimize for small code size.
opt-level = "s"
```

│ ├─ 📄 lib.rs

```rust,no_run
mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, hello-wasm!");
}
 println!("hello world!");
}
```

│ └─ 📄 utils.ra

```rust,no_run
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
```

## Next

Let's continue to [Enjoy ➠](./enjoy1.md)
