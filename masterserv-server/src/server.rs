use std::time::Duration;

use masterserv::{DummyGame};
use tokio::{task::JoinHandle, time::{sleep}};

use crate::{HostManager, WSServer};

pub struct Server {
    pub host_manager:HostManager,
}

impl Server {
    pub fn new() -> Self {
        let mut server = Server {
            host_manager:Default::default()
        };

        server.host_manager.register_game_type::<DummyGame>();

        return server;
    }

    pub fn spawn(mut self) -> JoinHandle<()> {
        return tokio::spawn(async move {
            println!("Spawning Server");
            for i in 0..1000 {
                self.host_manager.spawn_host("DummyGame");
            }
            loop {
                sleep(Duration::from_millis(1)).await;
            }
        });
    }
}