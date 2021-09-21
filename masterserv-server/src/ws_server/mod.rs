use masterserv::log::info;
use tokio::{net::TcpListener, sync::mpsc::{UnboundedSender}, task::JoinHandle};

use crate::HostServerMsg;


#[derive(Default)]
pub struct WSServer { 
    addr:String,
    host_manager:Option<UnboundedSender<HostServerMsg>>
}

impl WSServer {
    pub fn new(addr:String) -> Self {
        Self {
            addr,
            host_manager:Default::default()
        }
    }
    pub fn set_host_manager(&mut self, host_manager:UnboundedSender<HostServerMsg>) {
        self.host_manager = Some(host_manager);
    }
    pub fn spawn(self) -> JoinHandle<()> {
        return tokio::spawn(async move {
            info!("Spawned");
            let try_socket = TcpListener::bind(&self.addr).await;
            let listener = try_socket.expect("WSServer: Failed to bind!");
        
            while let Ok((stream, _)) = listener.accept().await {
                //tokio::spawn(accept(stream));
            }
        });
    }
}