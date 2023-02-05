# Serialize Js data

![](/assets/kat.png) Here's fun facts.

1. Send `JsValue` from `js` and use [`serde_wasm_bindgen::from_value(val)`](https://rustwasm.github.io/wasm-bindgen/reference/arbitrary-data-with-serde.html#receive-it-from-javascript-with-serde_wasm_bindgenfrom_value) to cast to derived `serde` struct.
1. Send `JSON` from `js` and use `[value.into_serde()`](https://rustwasm.github.io/wasm-bindgen/reference/arbitrary-data-with-serde.html#an-alternative-approach---using-json) to cast `serde` struct in `Rust`.
1. Send `JSON.stringify(json)` from `js` and use serde [`deserialize_with`](https://serde.rs/stream-array.html). // Not recommend but sometime we have to accept this from other.

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

    let u8s = tx_value
        .signatures
        .into_iter()
        .map(|e| {
            e.into_values()
                .map(|v| v.as_u64().unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>();

    let tx = Transaction {
        signatures: u8s[0].clone(),
    };

    println!("{tx:#?}");
    assert_eq!(tx.signatures, vec![16, 42]);
}
```

### With `Result`

```rust,edition2021
# use serde::Deserialize;
# use serde_json::{json, Value};
# use std::collections::HashMap;
#
# #[derive(Deserialize, Debug)]
# struct TransactionValue {
#     signatures: Vec<HashMap<String, Value>>,
# }
#
# #[derive(Deserialize, Debug)]
# struct Transaction {
#     signatures: Vec<u8>,
# }
#
# fn main() {
#     let value = json!({
#         "signatures": [{"0":16,"1":42}]
#     });
#     let tx_value = serde_json::from_value::<TransactionValue>(value).unwrap();
#     println!("{tx_value:#?}");

    let u8s = tx_value
        .signatures
        .into_iter()
        .map(|e| {
            e.into_values()
                .map(|v| {
                    v.as_u64()
                        .map(|num| num as u8)
                        .ok_or(format!("expected u64, but got {:?}", v))
                })
                .collect::<Result<Vec<u8>, String>>()
        })
        .collect::<Result<Vec<_>, String>>();

#     let tx = match u8s {
#         Ok(u8s) => Transaction {
#             signatures: u8s[0].clone(),
#         },
#         Err(e) => {
#             println!("Error: {}", e);
#             return;
#         }
#     };

#     println!("{tx:#?}");
#     assert_eq!(tx.signatures, vec![16, 42]);
# }
```

## Can we do better?

- Use `#[serde(deserialize_with = "deserialize_signatures")]` and/or `#[serde(rename(deserialize = "signatures"))]` as [ref](https://serde.rs/stream-array.html).
- Use `impl TryFrom` as [ref](https://doc.rust-lang.org/rust-by-example/conversion/try_from_try_into.html).
- Use `thiserror` when failures as [ref](https://docs.rs/thiserror/latest/thiserror/).
