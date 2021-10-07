mod hosted;
use client::Client;
pub use hosted::*;

use wasm_bindgen::prelude::*;
use js_sys::Uint8Array;
use masterserv::log::info;
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
            send(&[0, 0, 0]);
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

#[wasm_bindgen]
pub fn connected() {
    info!("Connected!");
}

#[wasm_bindgen]
pub fn disconnected() {
    info!("DisConnected!");
}

#[wasm_bindgen]
pub fn message(data:&[u8]) {
    info!("message: {}", data.len());
}

#[wasm_bindgen]
extern "C" {
    pub fn send(data:&[u8]);
}
