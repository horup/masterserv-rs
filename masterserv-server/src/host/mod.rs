use std::time::Duration;

use masterserv::{Context, HostMsg, log::{info}};

mod handle;
pub use handle::*;
use tokio::time::interval;

pub struct Host {
    handle: HostHandle,
}

impl Host {
    pub fn new(handle: HostHandle) -> Self {
        Host { handle }
    }

    pub fn spawn(mut self) {
        tokio::spawn(async move {
            info!("{:?} Spawned with name '{}' and game type '{}'", self.handle.id, self.handle.name, self.handle.game_type_name);
            self.run().await;
            info!("{:?} Terminated", self.handle.id);
        });
    }

    fn pop_messages(&mut self) -> Vec<HostMsg> {
        let mut new_messages = Vec::new();
        if let Ok(mut messages) = self.handle.messages.lock() {
            new_messages = messages.clone();
            messages.clear();
        }

        return new_messages;
    }

    async fn run(&mut self) {
        let create_game = self.handle.create_game.clone();
        let mut game = create_game();
        let period = Duration::from_millis(1000 / game.tick_rate());
        let mut timer = interval(period);
        let mut run = true;
        
        game.start(self.handle.id, self.handle.name.clone());
        while run {
            let host_messages = self.pop_messages();
            for msg in &host_messages {
                match msg {
                    HostMsg::Terminate => run = false,
                    _ => {}
                }
            }

            let mut context = Context {
                delta_seconds:period.as_secs_f32(),
                messages_from_host:host_messages,
                messages_from_game:Vec::new()
            };

            game.update(&mut context);

            for _msg in context.messages_from_game {
                
            }

            timer.tick().await;
        }
    }
}
