use uuid::Uuid;
use crate::{HostMsg, GameMsg};

pub struct Context {
    pub delta_seconds:f32,
    pub messages_from_game:Vec<GameMsg>,
    pub messages_from_host:Vec<HostMsg>
}

impl Context {
    pub fn push_message(&mut self, game_msg:GameMsg) {
        self.messages_from_game.push(game_msg);
    }
}

pub trait HostedGame : Send {

    fn start(&mut self, _id:Uuid, _name:String) {

    }

    fn stop(&mut self) {

    }
    
    fn default_max_players(&self) -> u32 {
        return 8;
    }

    fn update(&mut self, context:&mut Context);

    fn tick_rate(&self) -> u64 {
        return 20;
    }
}

pub trait GameType : HostedGame + Default + 'static {
    const NAME:&'static str;
}