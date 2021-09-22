use masterserv::uuid::Uuid;

#[derive(Debug)]
pub enum HostServerMsg {
    SpawnHost {
        id:Uuid,
        name:String,
        game_type:String
    },
    TerminateHost {
        id:Uuid
    }
}