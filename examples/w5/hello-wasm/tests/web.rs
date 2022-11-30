//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

// Running Tests in Headless Browsers
wasm_bindgen_test_configure!(run_in_browser);

// Import function to test.
use hello_wasm::greet;

#[wasm_bindgen_test]
fn test_greet() {
    // Call greet and get output.
    let output_js_value = greet("world");

    // Validate.
    assert_eq!("Hello world".to_string(), output_js_value)
}
