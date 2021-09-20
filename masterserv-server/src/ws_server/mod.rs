use tokio::{net::TcpListener, task::JoinHandle};


#[derive(Default)]
pub struct WSServer { 
    pub addr:String
}


impl WSServer {
    pub fn new(addr:String) -> Self {
        Self {
            addr
        }
    }
    pub fn spawn(self) -> JoinHandle<()> {
        return tokio::spawn(async move {
            let try_socket = TcpListener::bind(&self.addr).await;
            let listener = try_socket.expect("WSServer: Failed to bind!");
        
            while let Ok((stream, _)) = listener.accept().await {
                //tokio::spawn(accept(stream));
            }
        });
    }
}