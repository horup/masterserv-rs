use std::marker::PhantomData;

use crate::HostedGame;

pub struct Host<T: HostedGame> {
    pub name: String,
    pub current_players: u32,
    pub max_players: u32,
    data: PhantomData<T>,
}

impl<T: HostedGame> Host<T> {
    pub fn new(name: &str) -> Self {
        Host {
            name: name.into(),
            current_players: 0,
            max_players: 98,
            data: PhantomData::default(),
        }
    }
}
