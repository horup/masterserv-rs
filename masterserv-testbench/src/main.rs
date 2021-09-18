use masterserv_server::Server;

#[tokio::main]
async fn main() {
    println!("Starting testbench");

    let server = Server::new();
    server.spawn();

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

    print!("Ending testbench...");
}
