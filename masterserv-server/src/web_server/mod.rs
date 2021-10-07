use std::{net::SocketAddr, str::FromStr};

use tokio::task::JoinHandle;
use warp::Filter;

use crate::Bus;

pub struct WebServer {
    addr:String,
    pub bus:Bus
}

impl WebServer {
    pub fn new(addr:String, bus:Bus) -> Self {
        Self {
            addr,
            bus
        }
    }

    pub fn spawn(self) -> JoinHandle<()> {
        return tokio::spawn(async move {
            let addr = SocketAddr::from_str(&self.addr).expect("Could not parse address");
            let routes = warp::any().map(|| "Hello, World!");
            //warp::serve(routes)
            warp::serve(warp::fs::dir("./public"))
            .run(addr).await;
        });
    }
}