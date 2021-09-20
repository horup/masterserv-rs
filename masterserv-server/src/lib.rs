/*use tokio::net::{TcpListener, TcpStream};
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:8080";

    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");

    while let Ok((stream, _)) = listener.accept().await {
        println!("TCP Socket connected");
        tokio::spawn(accept(stream));
    }
}

async fn accept(stream:TcpStream) {
    let stream = tokio_tungstenite::accept_async(stream)
    .await;

    match stream {
        Ok(stream) => {
            let (mut tx, _rx) = stream.split();
            println!("TCP WebSocket connected");
            let buffer = vec![0;102400];
            loop {
                //let res = tx.send(Message::Text("hello world".into())).await;
                let res = tx.send(Message::binary(buffer.clone()));
                if let Err(err) = res.await {
                    println!("{:?}", err);
                    return;
                }
            }
        },
        Err(_) => {
            return;
        },
    }
}*/

mod ws_server;
pub use ws_server::*;

mod server;
pub use server::*;

mod host_manager;
pub use host_manager::*;


