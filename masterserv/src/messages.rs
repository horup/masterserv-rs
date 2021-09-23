use crate::Player;

#[derive(Clone, Debug)]
pub enum HostMsg {
    PlayerJoined(Player),
    PlayerLeft(Player),
    Terminate,
    PlayerMsg(Vec<u8>)
}

pub enum PlayerMsg {
    PlayerJoined(Player),
    PlayerLeft(Player),
    Terminate,
    GameMsg(Vec<u8>)
}