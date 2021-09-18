use std::{cell::RefCell, rc::Rc, sync::Arc, time::Instant};

use futures_util::{StreamExt, TryStreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() {
    if let Ok(stream) = TcpStream::connect("127.0.0.1:8080").await {
        println!("Connected to Socket!");
        let stream = tokio_tungstenite::connect_async("wss://127.0.0.1:8080/some/kind/of/url").await;
        let bytes = Rc::new(RefCell::new(0));
        let mut now = Instant::now();
        let secs = Rc::new(RefCell::new(0));
        if let Ok((stream, response)) = stream {
            println!("{:?}", response);
            println!("Connected to WebSocket!");
            let (mut tx, rx) = stream.split();

            let for_each = rx.try_for_each(|msg| async {
                match msg {
                    Message::Binary(s) => {
                        //println!("{}", s);
                        let mut chars = bytes.borrow_mut();
                        let mut secs = secs.borrow_mut();
                        *chars += s.len();
                        //*chars += s.len();
                        let elapsed = now.elapsed();

                        if elapsed.as_secs() > *secs {
                            let chars_per_sec = *chars as f64 / elapsed.as_secs_f64();
                            println!("{} MB/s", chars_per_sec / 1024.0 / 1024.0);
                            *secs = elapsed.as_secs();
                        }
                    },
                    _ => {}
                }

               Ok(())
            });

            let _ = for_each.await;
            
        } else if let Err(err) = stream {
            println!("{:?}", err);
        }
    }

    println!("Bye Bye");
}
