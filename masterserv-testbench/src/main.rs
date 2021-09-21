use masterserv::{DummyGame, uuid::Uuid};
use masterserv_server::{HostServer, HostServerMsg, WSServer};

#[tokio::main]
async fn main() {
    println!("Starting testbench");

    // instantiate
    let mut ws_server = WSServer::new("0.0.0.0:8080".into());
    let mut host_manager = HostServer::new();

    // configure
    host_manager.register_game_type::<DummyGame>();
    let host_manager_tx = host_manager.tx.clone();
    ws_server.set_host_manager(host_manager.tx.clone());
    
    // spawn
    let host_manager = host_manager.spawn();
    let ws_server = ws_server.spawn();

    // make some hosts
    for i in 0..100 {
        let _ = host_manager_tx.send(HostServerMsg::SpawnHost {
            game_type:"DummyGame".into(),
            name:format!("Game {}", i),
            id:Uuid::new_v4()
        });
    }

    let mut clients = Vec::new();
    for i in 0..10 {
        let handle = tokio::spawn(async move {
            println!("Spawning Client {}", i);
        });

        clients.push(handle);
    }
 
    for client in clients {
        let _ = client.await;
    }


    let _ = host_manager.await;
    let _ = ws_server.await;

    println!("Ending testbench...");
}
