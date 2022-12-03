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
