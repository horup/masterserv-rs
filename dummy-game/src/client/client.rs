use crate::shared::state::GameState;

use super::platform::canvas::Canvas;


pub struct Client {
    canvas:Canvas,
    state:GameState
}

impl Client {
    pub fn new() -> Self {
        Self {
            canvas:Canvas::new(),
            state:GameState::new()
        }
    }

    pub fn draw(&self) {
        self.canvas.clear();
        self.canvas.fill_rect(self.state.x, self.state.x, 50.0, 50.0);
        self.canvas.fill_text("hello world", 0.0, 200.0);
    }

    pub fn update(&mut self) {
        self.state.x += 1.0;
        self.draw();
    }
}