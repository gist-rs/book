# Serialize Js data

![](/assets/kat.png) Here's fun facts.

1. Send `JsValue` from `js` and use [`serde_wasm_bindgen::from_value(js_value)`](https://rustwasm.github.io/wasm-bindgen/reference/arbitrary-data-with-serde.html#receive-it-from-javascript-with-serde_wasm_bindgenfrom_value) to cast to derived `serde` struct.

   ```mermaid
   graph LR
    A["Js <code>new Uint8Array([16, 42])</code>"] --JsValue<br><code>UInt8Array</code>--> B["Wasm <code>serde_wasm_bindgen::from_value(js_value)</code>"] --"Struct<br><code>&[u8]</code>"--> C[Rust <code>Wasm</code>]
   ```

2. Send `JSON.parse(str)` from `js` and use [`js_value.into_serde()`](https://rustwasm.github.io/wasm-bindgen/reference/arbitrary-data-with-serde.html#an-alternative-approach---using-json) to cast `serde` struct in `Rust`.

   ```mermaid
   graph LR
   A["Js <code>JSON.parse(str)</code>"] --"JsValue<br><code>[16,42]</code>"--> B["Wasm <code>js_value.into_serde()</code>"] --"Struct<br><code>&[u8]</code>"--> C[Rust <code>Wasm</code>]
   ```

3. Send `JSON.stringify(json)` from `js` and use serde [`deserialize_with`](https://serde.rs/stream-array.html). // Consider bad practice.

   ```mermaid
   graph LR
   A["Js <code>JSON.stringify(json)</code>"] --"String<br><code>[{'0':16,'1':42}]</code">--> B["Wasm <code>serde::deserialize_with</code>"] --"Struct<br><code>&[u8]</code>"--> C[Rust <code>Wasm</code>]
   ```

## How to handle JSON stringify `UInt8Array`?

![](/assets/kat.png) Somehow an app (actually `dApp`) tend to send us `Uint8Array` as `JSON` string like this...

```json
{
  "signatures": [
    {
      "0": 16,
      "1": 42
    }
  ]
}
```

![](/assets/kat.png) And yes you can convert it to `JsValue` before sending to `Wasm`.

```js
const u8 = new Uint8Array([16, 42]) // Uint8Array [16,42]
const u8_string = JSON.stringify(u8) // '{"0":16,"1":42}'
const u8_map = JSON.parse(u8_string) // {0: 16, 1: 42}
const parsed_u8 = new Uint8Array(Object.values(u8_map)) // Uint8Array [16,42]
```

![](/assets/kat.png) But if you tend to keep all logic in `Rust` so we will cry and accept `JSON` as `String` and convert it to `u8` in `Wasm` instead.

### With `unwrap`

> **Warning**: `unwrap` without handle is for POC or testing purpose.

```rust,editable,edition2021
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct TransactionValue {
    signatures: Vec<HashMap<String, Value>>,
}

#[derive(Deserialize, Debug)]
struct Transaction {
    signatures: Vec<u8>,
}

fn main() {
    let value = json!({
        "signatures": [{"0":16,"1":42}]
    });
    let tx_value = serde_json::from_value::<TransactionValue>(value).unwrap();
    println!("{tx_value:#?}");

    // 😱 This will be bug you because it won't sorted (randomly).
    let bug_u8s = tx_value
        .signatures
        .into_iter()
        .map(|e| {
            let mut keys: Vec<&String> = e.keys().collect();
            keys.sort();
            keys.into_iter().map(|k| e[k].as_u64().unwrap() as u8).collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>();
    println!("{bug_u8s:#?}");

    // 😅 This is sorted.
    let u8s = tx_value
        .signatures
        .into_iter()
        .map(|e| {
            let mut keys: Vec<&String> = e.keys().collect();
            keys.sort();
            keys.into_iter().map(|k| e[k].as_u64().unwrap() as u8).collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>();

    let tx = Transaction {
        signatures: u8s[0].clone(),
    };

    println!("{tx:#?}");
    assert_eq!(tx.signatures, vec![16, 42]);
}
```

## Can we do better?

- Use `#[serde(deserialize_with = "deserialize_signatures")]` and/or `#[serde(rename(deserialize = "signatures"))]` as [ref](https://serde.rs/stream-array.html).
- Use `impl TryFrom` as [ref](https://doc.rust-lang.org/rust-by-example/conversion/try_from_try_into.html).
- Use `thiserror` when failures as [ref](https://docs.rs/thiserror/latest/thiserror/).
