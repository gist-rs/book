mod utils;

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

// app
use reqwest::Client;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub async fn fetch() -> Result<JsValue, JsError> {
    set_panic_hook();

    let client = &Client::new();
    let url = "https://github.com/katopz";

    let resp = client.get(url).send().await?;
    let bytes = resp.bytes().await?;

    assert!(!bytes.is_empty());

    Ok(serde_wasm_bindgen::to_value(&bytes.len())?)
}
