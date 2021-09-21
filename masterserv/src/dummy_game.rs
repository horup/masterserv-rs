use std::time::Instant;

use log::info;
use uuid::Uuid;

use crate::{Game, GameType};

pub struct DummyGame {
    pub test:f32,
    pub missed_count:u32,
    pub start_time:Instant,
    pub ticks:u64
}

impl Default for DummyGame {
    fn default() -> Self {
        Self { test: Default::default(), missed_count: Default::default(), start_time: Instant::now(), ticks:0 }
    }
}

impl GameType for DummyGame {
    const NAME:&'static str = "DummyGame";
}

impl Game for DummyGame {
    fn start(&mut self, _id:Uuid, name:String) {
        info!("Starting DummyGame with name '{}'", name);
    }

    fn update(&mut self, _delta_sec:f32) {
        // do some work
       for y in 0..64 {
            for x in 0..64 {
                self.test = self.test + y as f32 * x as f32;
            }
        }

        // calc ticks per second and report if below target
        let ticks_per_second = self.ticks as f64 / (Instant::now() - self.start_time).as_secs_f64();
        if ticks_per_second < self.tick_rate() as f64 {
            self.missed_count += 1;
            if self.missed_count > 4 {
                println!("{}", ticks_per_second);
            }
        } else {
            self.missed_count = 0;
        }

        self.ticks += 1;
    }

    fn stop(&mut self) {

    }

    fn tick_rate(&self) -> u64 {
        return 20;
    }
}