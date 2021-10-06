use masterserv::log::info;
use tokio::{net::TcpListener, sync::{mpsc::{self}}};
use tokio::task::JoinHandle;

use crate::{Bus,};

mod msg;
pub use msg::*;

pub struct WSServer { 
    addr:String,
    pub bus:Bus
}

impl WSServer {
    pub fn new(addr:String, bus:Bus) -> Self {
        let (_tx, _rx) = mpsc::unbounded_channel::<WSServerMsg>();
        Self {
            addr,
            bus
        }
    }
    
    pub fn spawn(self) -> JoinHandle<()> {
        return tokio::spawn(async move {
            info!("Spawned");
            let try_socket = TcpListener::bind(&self.addr).await;
            let listener = try_socket.expect("WSServer: Failed to bind!");
        
            while let Ok((_stream, _)) = listener.accept().await {
                //tokio::spawn(accept(stream));
            }
        });
    }
}