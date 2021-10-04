use dummy_game::DummyGame;
use masterserv::log::{LevelFilter, info};
use masterserv::{uuid::Uuid};
use masterserv_server::{Bus, BusEvent, HostManager, WSServer};

#[tokio::main]
async fn main() {
    let bus = Bus::default();
    let ws_server = WSServer::new("0.0.0.0:8080".into(), bus.clone());
    let mut host_manager = HostManager::new(bus.clone());

    // configure
    host_manager.register_game_type::<DummyGame>();

    // spawn
    let host_manager = host_manager.spawn();
    let ws_server = ws_server.spawn();

    println!("hi");
    let _ = host_manager.await;
    let _ = ws_server.await;
}