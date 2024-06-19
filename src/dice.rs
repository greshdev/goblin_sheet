use leptos::logging::log;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::js_sys::{self};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
pub struct DiceResult {
    #[serde(rename = "groupId")]
    pub group_id: i64,
    #[serde(rename = "rollId")]
    pub roll_id: i64,
    pub sides: String,
    pub theme: String,
    #[serde(rename = "themeColor")]
    pub theme_color: String,
    pub value: i64,
}

#[wasm_bindgen(module = "/src/dice.js")]
extern "C" {
    pub async fn roll_dice(s: &str) -> JsValue;
}

pub async fn get_dice_result(s: &str) -> Vec<DiceResult> {
    let result = roll_dice(s).await;
    if let Ok(data) = serde_wasm_bindgen::from_value::<Vec<DiceResult>>(result)
    {
        data
    } else {
        log!("Could not deserialize dice results!!");
        vec![]
    }
}

#[wasm_bindgen]
pub fn rust_test(_input: JsValue) {
    log!("Hello from Rust land!")
}
