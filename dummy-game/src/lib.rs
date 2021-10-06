mod hosted;
use client::Client;
pub use hosted::*;

use wasm_bindgen::prelude::*;
mod client;

pub mod shared;


static mut GLOBAL_CLIENT:Option<Client> = None;

#[wasm_bindgen]
pub fn start() {
    wasm_logger::init(wasm_logger::Config::default());

    unsafe {
        let mut client = Client::new();
        client.init();
        GLOBAL_CLIENT = Some(client);
    }
}

#[wasm_bindgen]
pub fn update() {
    unsafe {
        if let Some(client) = &mut GLOBAL_CLIENT {
            client.update();
        }
    }
}

#[wasm_bindgen]
pub fn keyup(keycode:u32) {
    unsafe {
        if let Some(client) = &mut GLOBAL_CLIENT {
            client.keyup(keycode);
        }
    }
}


#[wasm_bindgen]
pub fn keydown(keycode:u32) {
    unsafe {
        if let Some(client) = &mut GLOBAL_CLIENT {
            client.keydown(keycode);
        }
    }
}

