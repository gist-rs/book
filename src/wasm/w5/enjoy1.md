# Enjoy Day 1

![](/assets/kat.png) They said `Wasm` should be fun, Let's do it!

> 💡 for more examples see 👉 [here](https://github.com/rustwasm/wasm-bindgen/tree/main/examples)

## JS Object in Wasm

```rust,no_run
// 👇 This allow us to bind rs with js
#[wasm_bindgen]
//      This & 👇 mean short-lived.
pub fn foo(bar: &JsValue) {
    // ...
}

#[wasm_bindgen]
//   This no & 👇 mean long-lived.
pub fn foo(bar: JsValue) {
    // ...
}
```

// 🚧 UNDER CONSTRUCTION
// If you can't wait try start [here](https://rustwasm.github.io/docs/book/)
