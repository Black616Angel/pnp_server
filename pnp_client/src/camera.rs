use crate::types::*;

use macroquad::prelude::*;
pub struct Camera {
    pub position: Vec2D,
    middle_mouse: MB,
}

impl Camera {
    pub fn new() -> Self {
        let middle_mouse = MB{ was_up: true, start: Vec2D::empty()};

        return Camera{position: Vec2D::empty(), middle_mouse}
    }

    pub fn x(&self) -> f32 {
        self.position.x
    }
    pub fn y(&self) -> f32 {
        self.position.y
    }

    pub fn movement(&mut self) -> Vec2D {
        if is_mouse_button_down(MouseButton::Middle){
            if self.middle_mouse.was_up {
                self.middle_mouse.was_up = false;
                self.middle_mouse.start = Vec2D::from(mouse_position());
            } else {
                self.position -= self.middle_mouse.start.clone() - mouse_position();
                self.middle_mouse.start = Vec2D::from(mouse_position()); 
            }
            // println!("1 Position: {},{}", middle_mouse.start.0,middle_mouse.start.1);
        } else if !self.middle_mouse.was_up && !is_mouse_button_down(MouseButton::Right){
            self.middle_mouse.was_up = true;
        }
        return self.position.clone()
    }
    
}