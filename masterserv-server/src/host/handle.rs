use std::sync::{Arc, Mutex};
use masterserv::{HostedGame, HostMsg, uuid::Uuid};

#[derive(Clone)]
pub struct HostHandle {
    pub id:Uuid,
    pub name:String,
    pub game_type_name:String,
    pub create_game:Arc<dyn Fn() -> Box<dyn HostedGame> + Sync + Send>,
    pub messages:Arc<Mutex<Vec<HostMsg>>>
}