use dummy_game::DummyGame;
use masterserv::log::{LevelFilter, info};
use masterserv::{uuid::Uuid};
use masterserv_server::{Bus, BusEvent, HostManager, WSServer};

#[tokio::main]
async fn main() {
    env_logger::builder().filter_level(LevelFilter::Debug).init();
    info!("Starting testbench");

    // instantiate
    let bus = Bus::default();
    let ws_server = WSServer::new("0.0.0.0:8080".into(), bus.clone());
    let mut host_manager = HostManager::new(bus.clone());

    // configure
    host_manager.register_game_type::<DummyGame>();
    
    // spawn
    let host_manager = host_manager.spawn();
    let ws_server = ws_server.spawn();

    // make some hosts
    for i in 0..3 {
        let id = Uuid::new_v4();
        let _ = bus.send.send(BusEvent::SpawnHost {
            host_id: id,
            name:format!("Game {}", i),
            game_type:"DummyGame".into(),
        });
    }

    let mut clients = Vec::new();
    for _i in 0..10 {
        let handle = tokio::spawn(async move {
        });

        clients.push(handle);
    }
 
    for client in clients {
        let _ = client.await;
    }

    let _ = host_manager.await;
    let _ = ws_server.await;

    info!("Ending testbench...");
}
