use masterserv::{GameMsg, uuid::Uuid};

#[derive(Debug)]
pub enum HostManagerNorthMsg {
    SpawnHost {
        id:Uuid,
        name:String,
        game_type:String
    },
    TerminateHost {
        id:Uuid
    }
}