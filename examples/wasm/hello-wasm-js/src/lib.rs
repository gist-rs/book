mod utils;

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

// Use `wee_alloc` as global allocator when feature is enabled
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn greet(something: &str) -> String {
    // Hook when panic (optional)
    set_panic_hook();

    // Return string
    format!("Hello {something}")
}

#[wasm_bindgen]
pub async fn async_greet(something: &str) -> Result<String, JsError> {
    // Hook when panic (optional)
    set_panic_hook();

    // Return result string
    Ok(format!("Hello {something}"))
}

#[wasm_bindgen]
pub async fn async_greet_js_value(something: &str) -> Result<JsValue, JsError> {
    // Hook when panic (optional)
    set_panic_hook();

    // Return result string
    Ok(JsValue::from_str(format!("Hello {something}").as_str()))
}

#[wasm_bindgen]
pub fn greet_js_error() -> Result<JsValue, JsError> {
    let js_error = JsError::new("hello error!");

    // `wasm_bindgen::JsError` doesn't implement `std::fmt::Debug`
    // 😱 uncomment below 👇 and you'll get the error 👆
    // println!("{js_error:#?}");

    Err(js_error)
}
