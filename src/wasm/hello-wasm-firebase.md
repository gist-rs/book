# Hello Wasm Firebase

## TL;DR

https://github.com/katopz/hello-firebase-wasm

## Initialize

```bash
# Setup Firebase
npm install -g firebase-tools
firebase init
firebase login

# Setup Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Setup Cargo watch
cargo install cargo-watch

# Setup WASM
cargo install -f wasm-bindgen-cli
```

## Develop

```bash
# To watch and build WASM from Rust
cd wasm
cargo watch -- . ./dev.sh

# To run WASM with Firebase
cd functions
npm run dev
```
