use uuid::Uuid;
pub struct Client {
    id:Uuid,
}

impl Client {
    pub fn new(id:Option<Uuid>) -> Self {
        let id = id.unwrap_or(Uuid::new_v4());
        Self {
            id,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}

pub enum ClientState {
    NotConnected,
}
