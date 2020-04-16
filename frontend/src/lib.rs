#![recursion_limit = "256"]

mod app;
mod fermentation;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    web_logger::init();

    yew::start_app::<app::App>();

    Ok(())
}
