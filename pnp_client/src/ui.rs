use crate::types::*;

use macroquad::prelude::*;

pub struct UIList {
    buttons: Vec<Button>,
}

impl UIList {
    pub fn new() -> Self {
        Self{ buttons: Vec::new() }
    }

    pub fn add_button(&mut self, btn: Button) {
        self.buttons.push(btn);
    }

    pub fn click(&mut self) {
        for button in &self.buttons {
            if button.is_clicked(Vec2D::from(mouse_position())) {
                button.click();
            }
        }
    }

    pub fn draw(&mut self) {
        for button in &self.buttons {
            button.draw();
        }
    }
}

pub struct Button {
    text: String,
    position: Vec2D,
    size: Vec2D,
    color: Color,
    fun: fn(),
}

impl Button {
    pub fn new(text: String, position: Vec2D, size: Vec2D, color: Color, fun: fn()) -> Self {
        Button { text, position, size, color, fun }
    }

    pub fn draw(&self) {
        let text_size = (self.size.y * 0.8) as u16;
        let mut font_scale = 1_f32;
        while measure_text(&self.text, Some(Font::default()), text_size, font_scale).width > (self.size.x * 0.8) {
            font_scale = font_scale * 0.8;
        }

        let text_size = text_size as f32;
        let text_x = self.position.x + self.size.x * 0.1;
        let text_y = self.position.y + self.position.y * 0.1 + text_size;

        draw_rectangle(self.position.x, self.position.y, self.size.x, self.size.y, self.color);
        draw_text(&self.text, text_x, text_y, text_size, Color { r: 0_f32, g: 0_f32, b: 0_f32, a: 255_f32 });
    }

    pub fn click(&self) {
        let fun = self.fun;
        fun();
    }

    pub fn is_clicked(&self, position: Vec2D) -> bool {
        let origin = self.position.clone();
        let end = self.position.clone() + self.size.clone();
        return origin <= position && end >= position;
    }
}