use std::collections::HashMap;

use masterserv::uuid::Uuid;
use masterserv::{Context, GameType, HostedGame, Player};
use masterserv::{log::info};

use crate::shared::state::GameState;

pub struct DummyGame {
    pub current_state:GameState,
    pub players:HashMap<Uuid, Player>
}

impl Default for DummyGame {
    fn default() -> Self {
        Self {
            current_state:GameState::demo(),
            players:HashMap::new()
        }
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
        for msg in &context.messages_from_host {
            match msg {
                masterserv::HostMsg::PlayerJoined(player) => {
                    info!("Player {} joined the game.", {player.id});
                    self.players.insert(player.id, player.clone());
                },
                masterserv::HostMsg::PlayerLeft(player) => {
                    info!("Player {} left the game.", {player.id});
                    self.players.remove(&player.id);
                },
                masterserv::HostMsg::Terminate => todo!(),
                masterserv::HostMsg::FromPlayer(_, _) => todo!(),
            }
        }
    }

    fn stop(&mut self) {

    }

    fn tick_rate(&self) -> u64 {
        return 20;
    }
}