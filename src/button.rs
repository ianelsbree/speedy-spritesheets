use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;

pub struct Button {
    rect: Rect,
    stroke_color: Color,
    fill_color: Color,
    function: Box<dyn FnMut()>,
}

impl Button {
    pub fn new(
        rect: Rect,
        stroke_color: Color,
        fill_color: Color,
        function: Box<dyn FnMut()>,
    ) -> Self {
        Self {
            rect,
            stroke_color,
            fill_color,
            function,
        }
    }
    fn call(&mut self) {
        (self.function)();
    }
    pub fn try_click(&mut self, point: Point) {
        if self.rect.contains_point(point) {
            self.call();
        }
    }
    pub fn render(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(self.fill_color);
        canvas.fill_rect(self.rect)?;
        canvas.set_draw_color(self.stroke_color);
        canvas.draw_rect(self.rect)?;

        Ok(())
    }
    pub fn rect(&self) -> Rect {
        self.rect
    }
}
