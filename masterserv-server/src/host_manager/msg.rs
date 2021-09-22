use masterserv::uuid::Uuid;

#[derive(Debug)]
pub enum HostManagerMsg {
    SpawnHost {
        id:Uuid,
        name:String,
        game_type:String
    },
    TerminateHost {
        id:Uuid
    }
}