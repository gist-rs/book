mod utils;

use serde_json::{json, Value};
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

// app

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// 1️⃣

#[wasm_bindgen]
pub fn hello() -> JsValue {
    serde_wasm_bindgen::to_value(&json!({
        "foo": "bar"
    }))
    .unwrap_or(JsValue::NULL)
}

// 2️⃣

use serde::{Deserialize, Serialize};

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
struct FooBar {
    foo: String,
}

#[allow(non_snake_case)]
#[wasm_bindgen]
pub async fn getFooBar(input: &str) -> Result<JsValue, JsError> {
    match serde_wasm_bindgen::to_value(&json!(FooBar {
        foo: format!("hi! {input}")
    })) {
        Ok(js_value) => Ok(js_value),
        Err(err) => Err(JsError::new(err.to_string().as_ref())),
    }
}

// 3️⃣

#[allow(non_snake_case)]
#[wasm_bindgen]
pub async fn fetchJsValue() -> Result<JsValue, JsError> {
    let value = match fetch_value().await {
        Ok(value) => value,
        Err(err) => return Err(JsError::new(err.to_string().as_str())),
    };

    match serde_wasm_bindgen::to_value(&json!(value)) {
        Ok(js_value) => Ok(js_value),
        Err(err) => return Err(JsError::new(err.to_string().as_ref())),
    }
}

pub async fn fetch_value() -> anyhow::Result<Value> {
    set_panic_hook();

    let resp = reqwest::get("https://raw.githubusercontent.com/gist-rs/book/main/examples/wasm/w5/hello-world/src/hello.json")
        .await?
        .json::<serde_json::Value>()
        .await?;

    println!("resp:{:#?}", resp);

    Ok(resp)
}

/*** md
## Output
```
1️⃣ hello: {foo: 'bar'}
2️⃣ getFooBar: {foo: 'hi! baz'}
3️⃣ fetchJsValue: {foo: 'bar'}
```
***/

#[cfg(test)]
mod test {
    use crate::fetch_value;

    #[tokio::test]
    async fn test_fetch_value() {
        let res = fetch_value().await;
        assert!(res.is_ok());
    }
}
