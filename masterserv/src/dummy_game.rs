use crate::{Game, GameType};

#[derive(Default)]
pub struct DummyGame {
    pub test:f32
}

impl GameType for DummyGame {
    const NAME:&'static str = "DummyGame";
}

impl Game for DummyGame {
    fn start(&mut self) {
        //println!("Starting DummyGame");
    }

    fn update(&mut self, delta_sec:f32) {
        for y in 0..64 {
            for x in 0..64 {
                self.test = self.test + y as f32 * x as f32;
            }
        }

        let expectation = 1000.0 / self.tick_rate() as f32 / 1000.0;
        if delta_sec > expectation {
            println!("{}", delta_sec);
        }
    }

    fn stop(&mut self) {

    }

    fn tick_rate(&self) -> u64 {
        return 20;
    }

    
}