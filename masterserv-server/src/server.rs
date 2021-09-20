use core::panic;
use std::{collections::HashMap, sync::{Arc, Mutex}, time::{Duration}};
use masterserv::{DummyGame, Game, GameType, uuid};
use tokio::{sync::{mpsc, oneshot}, task::JoinHandle, time::{Instant, sleep, sleep_until}};

use crate::{HostHandle, HostMsg};

pub struct Server {
    pub game_types:HashMap<&'static str, Arc<dyn Fn()->Box<dyn Game> + Sync + Send>>,
    pub games:Vec<HostHandle>
}

impl Server {
    pub fn new() -> Self {
        let mut server = Server {
            game_types:Default::default(),
            games:Default::default()
        };

        server.register_game_type::<DummyGame>();

        return server;
    }

    pub fn register_game_type<T:GameType + Send>(&mut self) {
        if self.game_types.contains_key(T::NAME) {
            panic!("Game Type Name already registered!");
        }
        self.game_types.insert(T::NAME, Arc::new(|| {
            return Box::new(T::default());
        }));
    }

    pub fn tick(&mut self) {
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
                let mut next_tick = Instant::now() + Duration::from_millis(1000 / game.tick_rate());
                let mut last = Instant::now();
                game.start();
                loop {
                    let now = Instant::now();
                    next_tick = next_tick + Duration::from_millis(1000 / (game.tick_rate() + 1));
                    
                    // pop messages
                    let mut new_messages = Vec::new();
                    if let Ok(mut messages) = messages.lock() {
                        new_messages = messages.clone();
                        messages.clear();
                    }
                    
                    game.update((now - last).as_secs_f32());

                    sleep_until(next_tick).await;
                    last = now;
                }
            });
        }

    }

    pub fn spawn(mut self) -> JoinHandle<()> {
        return tokio::spawn(async move {
            println!("Spawning Server");
            for i in 0..1 {
                self.spawn_host("DummyGame");
            }
            loop {
                self.tick();
                sleep(Duration::from_millis(1)).await;
            }
        });
    }
}