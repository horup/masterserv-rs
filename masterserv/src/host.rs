use std::marker::PhantomData;

use crate::Game;

pub struct Host<T: Game> {
    pub name: String,
    pub current_players: u32,
    pub max_players: u32,
    data: PhantomData<T>,
}

impl<T: Game> Host<T> {
    pub fn new(name: &str) -> Self {
        Host {
            name: name.into(),
            current_players: 0,
            max_players: T::default_max_players(),
            data: PhantomData::default(),
        }
    }
}
