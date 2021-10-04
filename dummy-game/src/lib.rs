mod hosted;
pub use hosted::*;

use wasm_bindgen::prelude::*;
use masterserv::log::info;

#[wasm_bindgen]
pub fn start() {
    wasm_logger::init(wasm_logger::Config::default());
    info!("hi there!");
}