use uuid::Uuid;
use crate::{HostMsg, PlayerMsg};

pub struct Context {
    pub delta_seconds:f32,
    pub messages_to_player:Vec<PlayerMsg>,
    pub messages_from_host:Vec<HostMsg>
}

pub trait HostedGame : Send {

    fn start(&mut self, _id:Uuid, _name:String) {

    }

    fn stop(&mut self) {

    }
    
    fn default_max_players(&self) -> u32 {
        return 8;
    }

    fn update(&mut self, context:Context);

    fn tick_rate(&self) -> u64 {
        return 20;
    }
}

pub trait GameType : HostedGame + Default + 'static {
    const NAME:&'static str;
}