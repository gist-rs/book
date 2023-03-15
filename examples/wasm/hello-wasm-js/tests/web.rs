//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use wasm_bindgen::*;
use wasm_bindgen_test::*;

// Running Tests in Headless Browsers
wasm_bindgen_test_configure!(run_in_browser);

// Import function to test.
use hello_wasm::*;
use serde_wasm_bindgen::from_value;

#[wasm_bindgen_test]
fn test_greet() {
    // Call greet and get an output.
    let output_string = greet("world");

    // Validate.
    assert_eq!("Hello world".to_string(), output_string);
}

#[wasm_bindgen_test]
async fn test_async_greet() {
    // Call greet and get an output.
    let output_string = async_greet("world").await.ok().unwrap();
    console_log!("{output_string}");

    // Validate.
    assert_eq!("Hello world".to_string(), output_string);
}

#[wasm_bindgen_test]
async fn test_async_greet_js_value() {
    // Call greet and get an output.
    let output_value = async_greet_js_value("world").await.ok().unwrap();

    // Validate.
    let output_string = from_value::<String>(output_value).ok().unwrap();
    assert_eq!("Hello world".to_string(), output_string);
}

#[wasm_bindgen_test]
async fn test_async_greet_js_error() {
    // Call greet and get an error.
    let js_error = greet_js_error();

    // `wasm_bindgen::JsError` cannot be formatted using `{:?}` because it doesn't implement `Debug`
    // 😱 uncomment below 👇 will get above error 👆
    // console_log!("{js_error:?}");

    // Convert JsError to JsValue.
    let js_value = JsValue::from(js_error.err());

    // And now we can Debug.
    console_log!("{js_value:?}");

    // Validate.
    assert!(format!("{js_value:?}").contains("hello error!"));
}
