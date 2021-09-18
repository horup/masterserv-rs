use std::marker::PhantomData;
use crate::{Game, Socket};
use uuid::Uuid;
pub struct Client<G:Game, S:Socket> {
    id:Uuid,
    socket:S,
    game:PhantomData<G>
}

impl<G:Game, S:Socket> Client<G, S> {
    pub fn new(id:Option<Uuid>) -> Self {
        let id = id.unwrap_or(Uuid::new_v4());
        Self {
            id,
            socket:S::default(),
            game:PhantomData::default()
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}

pub enum ClientState {
    NotConnected,
}
