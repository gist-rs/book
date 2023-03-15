mod utils;

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn greet(something: &str) -> String {
    // Hook when panic (optional)
    set_panic_hook();

    // Return String
    format!("Hello {something}")
}

#[wasm_bindgen]
pub async fn async_greet(something: &str) -> Result<String, JsError> {
    // Hook when panic (optional)
    set_panic_hook();

    // Return Result String
    Ok(format!("Hello {something}"))
}

#[wasm_bindgen]
pub async fn async_greet_js_value(something: &str) -> Result<JsValue, JsError> {
    // Hook when panic (optional)
    set_panic_hook();

    // Return Result String
    Ok(JsValue::from_str(format!("Hello {something}").as_str()))
}

#[wasm_bindgen]
pub fn greet_js_error() -> Result<JsValue, JsError> {
    let js_error = JsError::new("hello error!");

    // `wasm_bindgen::JsError` doesn't implement `std::fmt::Debug`
    // the trait `std::fmt::Debug` is not implemented for `wasm_bindgen::JsError`
    // 😱 uncomment below 👇 will get above error 👆
    // println!("{js_error:#?}");

    Err(js_error)
}
