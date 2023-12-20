use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;

use crate::button::Button;

pub struct Component {
    area: Rect,
    background_color: Color,
    canvas: WindowCanvas,
    button_list: Vec<Button>,
}

impl Component {
    pub fn new(area: Rect, canvas: WindowCanvas, background_color: Color) -> Self {
        Component {
            area,
            background_color,
            canvas,
            button_list: Vec::new(),
        }
    }
    pub fn render(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(self.background_color);
        self.canvas.fill_rect(self.area)?;
        for button in &self.button_list {
            button.render(&mut self.canvas)?;
        }
        self.canvas.present();
        Ok(())
    }
    pub fn add_button(&mut self, button: Button) {
        self.button_list.push(button);
    }
    pub fn on_click(&mut self, point: Point) {
        for button in &mut self.button_list {
            button.try_click(point);
        }
    }
}
