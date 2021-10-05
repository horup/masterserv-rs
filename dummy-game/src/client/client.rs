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
        self.state = GameState::demo();
    }

    pub fn draw(&self) {
        self.canvas.clear();
        let grid_size = 16.0;
        

        // draw debug circle of things
        for (_, thing) in &self.state.things {
            let x = thing.pos.x as f64 * grid_size;
            let y = thing.pos.y as f64 * grid_size;
            self.canvas.draw_circle(x, y, thing.radius as f64 * grid_size);
        }

        // draw things
        for (_, thing) in &self.state.things {
            let x = thing.pos.x as f64 * grid_size;
            let y = thing.pos.y as f64 * grid_size;
            self.canvas.draw_image(0, x, y);
        }

         // draw names of things
         for (_, thing) in &self.state.things {
            let x = thing.pos.x as f64 * grid_size;
            let y = thing.pos.y as f64 * grid_size;
            self.canvas.fill_text("SOH", x, y);
        }
    }

    pub fn update(&mut self) {
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