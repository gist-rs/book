//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use wasm_bindgen_test::*;

// Running Tests in Headless Browsers
wasm_bindgen_test_configure!(run_in_browser);

// Import function to test.
use hello_wasm::{async_greet, greet};

#[wasm_bindgen_test]
fn test_greet() {
    // Call greet and get output.
    let output_string = greet("world");

    // Validate.
    assert_eq!("Hello world".to_string(), output_string)
}

#[wasm_bindgen_test]
async fn test_async_greet() {
    // Call greet and get output.
    let output_string = async_greet("world").await.ok().unwrap();

    // Validate.
    assert_eq!("Hello world".to_string(), output_string)
}
