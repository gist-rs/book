mod utils;

use serde_json::Value;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::wasm_bindgen_test;

// app

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub async fn fetch_js_value() -> Result<JsValue, JsError> {
    set_panic_hook();

    let resp = fetch_value().await?;

    println!("resp:{:#?}", resp);

    Ok(serde_wasm_bindgen::to_value(&resp)?)
}

pub async fn fetch_value() -> Result<Value, JsError> {
    set_panic_hook();

    let resp = reqwest::get("https://raw.githubusercontent.com/gist-rs/book/main/examples/wasm/w5/hello-world/src/hello.json")
        .await?
        .json::<serde_json::Value>()
        .await?;

    println!("resp:{:#?}", resp);

    Ok(resp)
}

#[cfg(test)]
mod test {
    use crate::fetch_value;

    #[tokio::test]
    async fn test_fetch_value() {
        let res = fetch_value().await;
        assert!(res.is_ok());
    }
}
