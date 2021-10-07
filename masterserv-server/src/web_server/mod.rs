use std::{net::SocketAddr, str::FromStr};

use futures_util::{FutureExt, StreamExt};
use tokio::task::JoinHandle;
use warp::{ws::Ws, Filter};

use crate::Bus;

pub struct WebServer {
    addr: String,
    pub bus: Bus,
}

impl WebServer {
    pub fn new(addr: String, bus: Bus) -> Self {
        Self { addr, bus }
    }

    pub fn spawn(self) -> JoinHandle<()> {
        return tokio::spawn(async move {
            let addr = SocketAddr::from_str(&self.addr).expect("Could not parse address");

            let public_route = warp::fs::dir("./public");

            let ws_route = warp::ws()
            .map(|ws: warp::ws::Ws| {
                ws.on_upgrade(|websocket| {
                    let (tx, rx) = websocket.split();
                    rx.forward(tx).map(|result| {
                        /*if let Err(e) = result {
                            eprintln!("websocket error: {:?}", e);
                        }*/
                    })
                })
            });
    
            let routes = warp::get()
            .and(ws_route)
            .or(public_route);

            warp::serve(routes).run(addr).await;
        });
    }
}
