# Enjoy Day 2

![](/assets/kat.png) Here's how to create `sync` and `async` function tha return `String`, `JsValue` and how to `test`.

## Structure.

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

#### `tests/index.html`

```html,no_run
{{#include ../../../examples/w5/hello-wasm/tests/index.html}}
```

## To test

```
wasm-pack test --headless --firefox
cp ./tests/index.html ./pkg
npx live-server ./pkg
```
