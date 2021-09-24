use std::time::Instant;
use log::info;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::{Context, GameMsg, GameType, HostedGame};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct DummyState{
    pub grid:[[u8;16];16]
}

#[derive(Serialize)]
pub enum DummyMsg {
    State(DummyState)
}

impl Default for DummyState {
    fn default() -> Self {
        Self { grid: [[0;16];16]}
    }
}

pub struct DummyGame {
    pub test:f32,
    pub missed_count:u32,
    pub start_time:Instant,
    pub ticks:u64,
    pub current_state:DummyState
}

impl Default for DummyGame {
    fn default() -> Self {
        Self { test: Default::default(), missed_count: Default::default(), start_time: Instant::now(), ticks:0, current_state:DummyState::default() }
    }
}

impl GameType for DummyGame {
    const NAME:&'static str = "DummyGame";
}

impl HostedGame for DummyGame {
    fn start(&mut self, _id:Uuid, name:String) {
        info!("Starting DummyGame with name '{}'", name);
    }

    fn update(&mut self, context:&mut Context) {
        // do some work
        for row in &mut self.current_state.grid {
            for col in row.iter_mut() {
                *col = 0;
            }
        }

        // push a custom message containing the game state
        if let Ok(encoded) = bincode::serialize(&self.current_state) {
            context.push_message(GameMsg::CustomAll(encoded));
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