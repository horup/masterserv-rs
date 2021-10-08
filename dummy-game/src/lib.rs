mod hosted;
use client::Client;
pub use hosted::*;

use wasm_bindgen::prelude::*;
use js_sys::Uint8Array;
use masterserv::{log::{error, info}, protocols::ServerMsg};
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
            client.server_messages.clear();

            for msg in &client.client_messages {
                match bincode::serialize(msg) {
                    Ok(v) => {
                        send(&v);
                    }
                    Err(v) => {
                        error!("{:?}", v);
                    }
                }
            }
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
    unsafe {
        if let Some(client) = &mut GLOBAL_CLIENT {
            client.connected();
        }
    }
}

#[wasm_bindgen]
pub fn disconnected() {
    unsafe {
        if let Some(client) = &mut GLOBAL_CLIENT {
            client.disconnected();
        }
    }
}

#[wasm_bindgen]
pub fn message(data:&[u8]) {
    unsafe {
        if let Some(client) = &mut GLOBAL_CLIENT {
            match bincode::deserialize::<ServerMsg>(data) {
                Ok(msg) => {

                }
                Err(err) => {
                    error!("{:?}", err);
                }
            }
        }
    }
}

#[wasm_bindgen]
extern "C" {
    pub fn send(data:&[u8]);
}
