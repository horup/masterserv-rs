use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub struct Canvas {
    context:CanvasRenderingContext2d,
    canvas:HtmlCanvasElement
}

impl Canvas {
    pub fn new() -> Canvas {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("primary").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        Canvas {
            context,
            canvas
        }
    }

    pub fn width(&self) -> u32 {
        self.canvas.width()
    }

    pub fn height(&self) -> u32 {
        self.canvas.height()
    }

    pub fn clear(&self) {
        self.context.clear_rect(0.0, 0.0, self.width() as f64, self.height() as f64);
    }

    pub fn fill_rect(&self, x:f64, y:f64, w:f64, h:f64) {
        self.context.fill_rect(x, y, w, h);
    }

    pub fn fill_text(&self, text:&str, x:f64, y:f64) {
        let _ = self.context.fill_text(text, x, y);
    }
}