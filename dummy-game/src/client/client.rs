use masterserv::log::info;

use crate::shared::state::GameState;

use super::platform::canvas::Canvas;


pub struct Client {
    canvas:Canvas,
    state:GameState
}

pub type KeyCode = u32;

impl Client {
    pub fn new() -> Self {
        Self {
            canvas:Canvas::new(),
            state:GameState::new()
        }
    }

    pub fn init(&mut self) {
        self.canvas.set_image_src(0, "dummy.png");
    }

    pub fn draw(&self) {
        self.canvas.clear();
        self.canvas.fill_rect(self.state.x, self.state.x, 50.0, 50.0);
        self.canvas.draw_image(0, self.state.x, self.state.x);
        self.canvas.fill_text("hello world", 0.0, 200.0);
    }

    pub fn update(&mut self) {
        self.state.x += 1.0;
        self.draw();
    }

    pub fn keyup(&mut self, code:KeyCode) {
    }

    pub fn keydown(&mut self, code:KeyCode) {
    }
}

unsafe impl Send for Client {
}
unsafe impl Sync for Client {
}