use dummy_game_lib::DummyGame;

use masterserv::{GameType, log::{LevelFilter, info}, uuid::Uuid};
use masterserv_server::{Bus, HostManager, WebServer};

#[tokio::main]
async fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();
    let bus = Bus::default();
    let web_server = WebServer::new("0.0.0.0:8080".into(), bus.clone());
    let mut host_manager = HostManager::new(bus.clone());

    // configure
    host_manager.register_game_type::<DummyGame>();

    host_manager.spawn_host(Uuid::new_v4(), DummyGame::NAME, "Default Game".into());

    // spawn
    let host_manager = host_manager.spawn();
    let web_server = web_server.spawn();

    info!("Starting server");
    let _ = host_manager.await;
    let _ = web_server.await;
}