use masterserv::log::info;
use tokio::{net::TcpListener, sync::{broadcast::{Receiver, Sender}, mpsc::{self, UnboundedReceiver, UnboundedSender}}, task::JoinHandle};

use crate::{Bus, BusEvent};

mod msg;
pub use msg::*;

pub struct WSServer { 
    addr:String,
    bus:Bus
}

impl WSServer {
    pub fn new(addr:String, bus:Bus) -> Self {
        let (tx, rx) = mpsc::unbounded_channel::<WSServerMsg>();
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