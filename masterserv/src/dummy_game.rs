use crate::{Game, GameType};

#[derive(Default)]
pub struct DummyGame {
}

impl GameType for DummyGame {
    const NAME:&'static str = "DummyGame";
}

impl Game for DummyGame {
    fn start(&mut self) {
        println!("creating dummy game");
    }

    fn update(&mut self, delta_sec:f32) {
        println!("DummyGame::update() Delta {}", delta_sec);
    }
}