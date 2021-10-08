use std::{net::SocketAddr, str::FromStr};

use futures_util::{FutureExt, SinkExt, StreamExt, stream::{SplitSink, SplitStream}};
use masterserv::{
    log::{error, info},
    protocols::{ClientMsg, ServerMsg},
    uuid::Uuid,
};
use tokio::task::JoinHandle;
use warp::{Filter, ws::{Message, WebSocket}};

use crate::Bus;

pub struct WebServer {
    addr: String,
    pub bus: Bus,
}

impl WebServer {
    pub fn new(addr: String, bus: Bus) -> Self {
        Self { addr, bus }
    }

    async fn client_joined(mut tx: SplitSink<WebSocket, Message>, mut rx:SplitStream<WebSocket>, bus:Bus, client_id:Uuid) {
        info!("Client {:?} joined", client_id);

        while let Some(msg) = rx.next().await {

        }
    }

    async fn client_connected(ws: WebSocket, bus: Bus) {
        
        let (mut tx, mut rx) = ws.split();

        let mut id = None;

        // wait for Hello message to get client id
        while let Some(msg) = rx.next().await {
            match msg {
                Ok(msg) => {
                    let bytes = msg.as_bytes();
                    if bytes.len() > 0 {
                        match bincode::deserialize::<ClientMsg>(bytes) {
                            Ok(msg) => match msg {
                                ClientMsg::Hello { client_id } => {
                                    id = Some(client_id);
                                    break;
                                }
                                _ => {}
                            },
                            Err(err) => {
                                error!("{:?}", err);
                                break;
                            }
                        }
                    }
                }
                Err(err) => {
                    error!("{:?}", err);
                    break;
                }
            }
        }

        if let Some(id) = id {
            let msg = ServerMsg::Welcome {

            };
            

            match tx.send(Message::binary(msg.to_bincode())).await {
                Ok(_) => Self::client_joined(tx, rx, bus, id).await,
                Err(_) => error!("Client {} failed to join", id),
            }
        }

        
        match id {
            Some(id) => info!("Client {} disconnected", id),
            None => info!("Unknown Client disconnected")
        }
    }

    pub fn spawn(self) -> JoinHandle<()> {
        return tokio::spawn(async move {
            let bus = self.bus.clone();
            let addr = SocketAddr::from_str(&self.addr).expect("Could not parse address");

            let public_route = warp::fs::dir("./public");

            let ws_route = warp::ws().map(move |ws: warp::ws::Ws| {
                let bus = bus.clone();
                ws.on_upgrade(move |ws| Self::client_connected(ws, bus))
            });

            let routes = warp::get().and(ws_route).or(public_route);

            warp::serve(routes).run(addr).await;
        });
    }
}
