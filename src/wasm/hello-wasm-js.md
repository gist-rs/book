# Hello Wasm Js

![](/assets/kat.png) Here's how to create `sync` and `async` function tha return `String`, `JsValue` and how to `test`.

## Structure.

#### `Cargo.toml`

```toml
{{#include ../../examples/wasm/hello-wasm-js/Cargo.toml}}
```

#### `utils.rs`

```rust,no_run
{{#include ../../examples/wasm/hello-wasm-js/src/utils.rs}}
```

#### `lib.rs`

```rust,no_run
{{#include ../../examples/wasm/hello-wasm-js/src/lib.rs}}
```

#### `tests/web.rs`

```rust,no_run
{{#include ../../examples/wasm/hello-wasm-js/tests/web.rs}}
```

#### `tests/index.html`

```html,no_run
{{#include ../../examples/wasm/hello-wasm-js/tests/index.html}}
```

## To test

```
wasm-pack test --headless --firefox
cp ./tests/index.html ./pkg
npx live-server ./pkg
```
