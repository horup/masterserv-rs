use core::panic;
use std::{collections::HashMap};
use masterserv::{DummyGame, Game, GameType};

pub struct Server {
    pub game_types:HashMap<&'static str, Box<dyn Fn()->Box<dyn Game> + Send>>,
    pub games:Vec<Box<dyn Game>>
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
        self.game_types.insert(T::NAME, Box::new(|| {
            return Box::new(T::default());
        }));
    }

    pub fn tick(&mut self) {
        self.start_game("DummyGame");
        println!("Ticking...");
    }

    pub fn start_game(&mut self, name:&str) -> bool {
        let game = self.game_types.get(name);
        if let Some(game) = game {
            let mut game = game();
            game.start();
            self.games.push(game);
        }

        false
    }

    pub fn spawn(mut self) {
        tokio::spawn(async move {
            println!("Spawning Server");
            loop {
                self.tick();

            }
        });
    }
}