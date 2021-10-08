use uuid::Uuid;
use crate::Player;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug)]
pub enum HostMsg {
    PlayerJoined(Player),
    PlayerLeft(Player),
    Terminate,
    FromPlayer(Uuid, Vec<u8>)
}

#[derive(Clone, Debug)]
pub enum GameMsg {
    CustomAll(Vec<u8>),
    ToPlayer(Uuid, Vec<u8>),
}
