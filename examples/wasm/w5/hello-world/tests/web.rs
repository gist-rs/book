//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use hello_world::fetch_js_value;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn fetch() {
    let res = fetch_js_value().await.unwrap();
    assert!(res.is_ok());
}
