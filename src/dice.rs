use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/dice.js")]
extern "C" {
    pub fn roll_dice(s: &str);
}
