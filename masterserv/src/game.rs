pub trait Game : Send {
    fn start(&mut self) {

    }

    fn stop(&mut self) {

    }
    
    fn default_max_players(&self) -> u32 {
        return 8;
    }

    fn update(&mut self, delta_sec:f32);

    fn tick_rate(&self) -> u32 {
        return 20;
    }
}

pub trait GameType : Game + Default + 'static {
    const NAME:&'static str;
}