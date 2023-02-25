# Serialize Js data

![](/assets/kat.png) Here's fun facts. let say we have this in `Js`.

```js
const u8 = new Uint8Array([16, 42]) // Uint8Array [16,42]
const u8_string = JSON.stringify(u8) // '{"0":16,"1":42}'
```

And want to send that `u8` from `Js` → `Wasm` → `Rust`.

## Our options

1. Send `Uint8Array` as `JsValue` from `js` and use [`serde_wasm_bindgen::from_value(js_value)`](https://rustwasm.github.io/wasm-bindgen/reference/arbitrary-data-with-serde.html#receive-it-from-javascript-with-serde_wasm_bindgenfrom_value) to cast to derived `serde` struct.

   ```mermaid
   graph LR
    A["<code>new Uint8Array([16, 42])</code>"] --JsValue<br><code>UInt8Array</code>--> B["<code>serde_wasm_bindgen::from_value(js_value)</code>"] --"Struct<br><code>&[u8]</code>"--> C[<code>Rust</code>]
   ```

2. Send `Array` as `JsValue` from `js` and use [`js_value.into_serde()`](https://rustwasm.github.io/wasm-bindgen/reference/arbitrary-data-with-serde.html#an-alternative-approach---using-json) to cast `serde` struct in `Rust`.

   ```mermaid
   graph LR
   A["<code>[16, 42]</code>"] --"JsValue<br><code>[16,42]</code>"--> B["<code>js_value.into_serde()</code>"] --"Struct<br><code>&[u8]</code>"--> C[<code>Rust</code>]
   ```

3. Send `JSON.stringify(new Uint8Array([16, 42]))` as `String/&str` from `js` and use serde [`deserialize_with`](https://serde.rs/stream-array.html).

   ```mermaid
   graph LR
   A["<code>JSON.stringify(new Uint8Array([16, 42]))</code>"] --"String<br><code>'{'0':16,'1':42}'</code">--> B["<code>serde::deserialize_with</code>"] --"Struct<br><code>&[u8]</code>"--> C[<code>Rust</code>]
   ```

## How to handle JSON stringify `UInt8Array`?

![](/assets/kat.png) Somehow an app (actually `dApp`) tend to send us `Uint8Array` as `JSON` string like this (fall into case #3 👆)...

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

![](/assets/kat.png) But if we tend to keep all logic in `Rust` so we cry and accept `JSON` as `String` then convert it to `u8` in `Wasm` instead.

### `Deserialize` with `serde`

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
        "signatures": [{"0":1,"1":2,"2":3,"3":4,"4":5,"5":6,"6":7,"7":8,"8":9,"9":10,"10":11}]
    });
    let tx_value = serde_json::from_value::<TransactionValue>(value).unwrap();
    println!("{tx_value:#?}");

    // 😱 This will be bug you because it won't sorted (randomly).
    let bug_u8s = tx_value
        .signatures
        .iter()
        .map(|e| {
            e.values()
                .map(|v| v.as_u64().unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>();
    println!("randomly bug:{bug_u8s:#?}");
    // assert_eq!(bug_u8s[0], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11,]);

    // 😱 This is sorted, but still bug because key is String.
    let still_bug_u8s = tx_value
        .signatures
        .iter()
        .map(|e| {
            let mut keys: Vec<&String> = e.keys().collect();
            keys.sort();
            keys.into_iter()
                .map(|k| e[k].as_u64().unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>();

    println!("sorted string bug:{still_bug_u8s:#?}");
    // assert_eq!(still_bug_u8s[0], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11,]);

    // 👍 This is sorted by usize.
    let u8s = tx_value
        .signatures
        .iter()
        .map(|e| {
            let mut keys = e
                .keys()
                .flat_map(|e| e.parse::<usize>())
                .collect::<Vec<_>>();

            keys.sort();
            keys.into_iter()
                .map(|k| e[&k.to_string()].as_u64().unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>();

    let tx = Transaction {
        signatures: u8s[0].clone(),
    };

    println!("{tx:#?}");
    assert_eq!(tx.signatures, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11,]);
}
```

## Can we do better?

- Use `#[serde(deserialize_with = "deserialize_signatures")]` and/or `#[serde(rename(deserialize = "signatures"))]` as [ref](https://serde.rs/stream-array.html).
- Use `impl TryFrom` as [ref](https://doc.rust-lang.org/rust-by-example/conversion/try_from_try_into.html).
- Use `thiserror` when failures as [ref](https://docs.rs/thiserror/latest/thiserror/).
- Use `borsh` and/or try from string slice.
