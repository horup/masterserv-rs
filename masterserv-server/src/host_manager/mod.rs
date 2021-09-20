use std::{collections::HashMap, sync::{Arc, Mutex}, time::{Duration, Instant}};
use masterserv::{Game, GameType, uuid};

mod host_handle;
pub use host_handle::*;

mod host_msg;
pub use host_msg::*;
use tokio::time::interval;

#[derive(Default)]
pub struct HostManager {
    pub game_types:HashMap<&'static str, Arc<dyn Fn()->Box<dyn Game> + Sync + Send>>,
    pub games:Vec<HostHandle>
}

impl HostManager {
    pub fn register_game_type<T:GameType + Send>(&mut self) {
        if self.game_types.contains_key(T::NAME) {
            panic!("Game Type Name already registered!");
        }
        self.game_types.insert(T::NAME, Arc::new(|| {
            return Box::new(T::default());
        }));
    }
    
    pub fn spawn_host(&mut self, game_type_name:&str) {
        let mut handle = HostHandle {
            id: uuid::Uuid::new_v4(),
            messages: Arc::new(Mutex::new(Vec::new()))
        };
      
        if let Some(game) = self.game_types.get(game_type_name) {
            let create_game = game.clone();
            tokio::spawn(async move {
                let messages = handle.messages.clone();
                let mut game = create_game();
                let period = Duration::from_millis(1000 / game.tick_rate());
                let mut next_tick = Instant::now() + Duration::from_millis(1000 / game.tick_rate());
                let mut last = Instant::now();
                let mut timer = interval(period);
                game.start();
                loop {
                    let now = Instant::now();
                    next_tick = next_tick + Duration::from_millis(1000 / (game.tick_rate()));
                    
                    // pop messages
                    let mut new_messages = Vec::new();
                    if let Ok(mut messages) = messages.lock() {
                        new_messages = messages.clone();
                        messages.clear();
                    }
                    
                    game.update(period.as_secs_f32());
                    timer.tick().await;
                    last = now;
                }
            });
        }
    }
}