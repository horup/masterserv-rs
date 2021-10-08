use uuid::Uuid;
use serde::{Serialize, Deserialize};



#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ClientMsg {
    Hello {
        client_id:Uuid
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ServerMsg {
    
}
