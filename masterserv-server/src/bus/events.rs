use masterserv::uuid::Uuid;

#[derive(Clone)]
pub enum BusEvent {
    ClientConnected {
        client_id:Uuid
    },
    ClientDisconnected {
        client_id:Uuid
    },
    SpawnHost {
        host_id:Uuid,
        name:String,
        game_type:String
    },
    TerminateHost {
        host_id:Uuid
    },
    ClientJoinedHost {
        host_id:Uuid
    },
    ClientLeftHost {
        host_id:Uuid
    }
}