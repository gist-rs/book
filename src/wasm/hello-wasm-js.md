# Hello Wasm Js

![](/assets/kat.png) <span class="speech-bubble">Here's how to create `sync` and `async` function in `Rust` which return `String`, `JsValue`, `JsError` and call it from `HTML`, `Javascript` in `test` via headless browser.</span>

## Fun facts

- 🚧 `println!` not working with `wasm-bindgen-test` context because `--nocapture` won't [passthrough](https://github.com/wasm-bindgen/wasm-pack/issues/730).
  > 💡 We have to use `console_log!` in `wasm` context.
- 🚧 `println!("{js_error:#?}")`and `console_log!("{js_error:#?}")` not working because [`JsError`](https://docs.rs/wasm-bindgen/latest/wasm_bindgen/struct.JsError.html) didn't implement `Debug`.
  > 💡 We have to convert [`JsError` to `JsValue`](https://docs.rs/wasm-bindgen/latest/wasm_bindgen/struct.JsError.html#impl-From%3CJsError%3E-for-JsValue) as workaround.
- 🚧 `#[tokio::test]` will break `wasm-bindgen-test`.
  > 💡 We need `#[cfg(not(target_arch = "wasm32"))]` above `Rust` context when needed.

## Structure

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
