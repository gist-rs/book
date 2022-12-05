## Dev

```
wasm-pack build --target web
```

## Test

```
# rs
cargo test

# wasm
wasm-pack test --headless --firefox

# js
cargo test
cp ./tests/index.html ./pkg
npx live-server ./pkg
open http://127.0.0.1:8080/
```

## Release

```
wasm-pack publish
```
