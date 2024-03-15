use leptos::logging::log;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/dice.js")]
extern "C" {
    pub fn roll_dice(s: &str);
}

#[wasm_bindgen]
pub fn rust_test(_input: JsValue) {
    log!("Hello from Rust land!")
}
