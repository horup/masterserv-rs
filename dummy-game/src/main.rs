use dummy_game_lib::DummyGame;

use masterserv::{GameType, log::{LevelFilter, info}, uuid::Uuid};
use masterserv_server::{HostManager, Bus, WSServer};

#[tokio::main]
async fn main() {
    env_logger::builder().filter_level(LevelFilter::Debug).init();
    let bus = Bus::default();
    let ws_server = WSServer::new("0.0.0.0:8080".into(), bus.clone());
    let mut host_manager = HostManager::new(bus.clone());

    // configure
    host_manager.register_game_type::<DummyGame>();

    host_manager.spawn_host(Uuid::new_v4(), DummyGame::NAME, "Default Game".into());

    // spawn
    let host_manager = host_manager.spawn();
    let ws_server = ws_server.spawn();

    info!("Starting server");
    let _ = host_manager.await;
    let _ = ws_server.await;
}