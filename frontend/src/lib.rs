#![recursion_limit = "2048"]

mod components;
mod data;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    web_logger::init();

    yew::start_app::<components::App>();

    Ok(())
}
