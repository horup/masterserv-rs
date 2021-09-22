use std::sync::{Arc, Mutex};

use masterserv::{Game, uuid::Uuid};

use crate::HostMsg;

#[derive(Clone)]
pub struct HostHandle {
    pub id:Uuid,
    pub name:String,
    pub game_type_name:String,
    pub create_game:Arc<dyn Fn() -> Box<dyn Game> + Sync + Send>,
    pub messages:Arc<Mutex<Vec<HostMsg>>>
}

