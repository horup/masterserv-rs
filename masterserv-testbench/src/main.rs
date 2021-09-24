use masterserv::log::{LevelFilter, info};
use masterserv::{DummyGame, uuid::Uuid};
use masterserv_server::{Bus, HostManager, HostManagerNorthMsg, WSServer};

#[tokio::main]
async fn main() {
    env_logger::builder().filter_level(LevelFilter::Debug).init();
    info!("Starting testbench");

    // instantiate
    let bus = Bus::default();
    let mut ws_server = WSServer::new("0.0.0.0:8080".into(), bus.clone());
    let mut host_manager = HostManager::new(bus.clone());

    // configure
    host_manager.register_game_type::<DummyGame>();
    let host_manager_tx = host_manager.tx.clone();
    ws_server.set_host_manager(host_manager.tx.clone());
    
    // spawn
    let host_manager = host_manager.spawn();
    let ws_server = ws_server.spawn();

    // make some hosts
    for i in 0..3 {
        let id = Uuid::new_v4();
        let _ = host_manager_tx.send(HostManagerNorthMsg::SpawnHost {
            game_type:"DummyGame".into(),
            name:format!("Game {}", i),
            id
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
