use glam::Vec2;

pub struct Thing {
    pub pos:Vec2,
    pub vel:Vec2
}

impl Thing {
    pub fn new() -> Self {
        Self {
            pos:[0.0, 0.0].into(),
            vel:[0.0, 0.0].into()
        }
    }
}


pub struct GameState {
    pub x:f64
}

impl GameState {
    pub fn new() -> Self
    {
        Self {
            x:0.0
        }
    }
}