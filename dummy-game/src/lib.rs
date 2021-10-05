mod hosted;
use std::{ptr, sync::Mutex};

use client::Client;
pub use hosted::*;

use wasm_bindgen::prelude::*;
mod client;

pub mod shared;



#[wasm_bindgen]
pub fn start() {
    wasm_logger::init(wasm_logger::Config::default());
    let mut client = Client::new();
    client.update();
}
