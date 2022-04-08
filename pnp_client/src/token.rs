use crate::{types::*, scene_json::{SceneJsonToken, ClickAction}};

use macroquad::prelude::*;

pub enum ClickMode {
    CLICKED,
    DRAGGED,
    NONE,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub position: Vec2D,
    pub size: Vec2D,
    texture: Texture2D,
    click_counter: i32,
    pos_grid: Vec2D,
    dragged: bool,
    click_action: ClickAction,
}

impl Token {

    pub async fn new_from_json(folder: String, json: SceneJsonToken, square_size: f32) -> Self {
        let pos_grid = Vec2D::from((json.position_x as f32, json.position_y as f32)) - 1_f32;
        let position = pos_grid * square_size; // - (square_size / 2) as f32;
        info!("");
        let texture = load_texture(&(folder + "/" + &json.texture_path)).await.unwrap();
        let size: Vec2D = Vec2D::from(((json.width) as f32, (json.height) as f32));
        if let Some(click_action) = json.click_action {
            Token{ position: position.clone(), size, texture, click_counter: 0, pos_grid, dragged: false, click_action }
        } else {
            Token{ position: position.clone(), size, texture, click_counter: 0, pos_grid, dragged: false, click_action: ClickAction::ShowStats("".to_string()) }
        }
    }
    
    pub fn in_token(&self, position: Vec2D, square_size: f32) -> bool {
        let origin = self.position.clone();
        let size = self.size * square_size;
        let end = self.position.clone() + size.clone();
        return origin <= position && end >= position;
    }

    pub fn movement(&mut self, direction: Vec2D) {
        self.position += direction;
    }

    pub fn final_move(&mut self, square_size: f32) {

        let rem = self.position.clone() % square_size;
        self.pos_grid.x = if rem.x > square_size / 2_f32 {
            (self.position.x - rem.x) / square_size + 1_f32
        } else {
            (self.position.x - rem.x) / square_size
        };
        self.pos_grid.y = if rem.y > square_size / 2_f32 {
            (self.position.y - rem.y) / square_size + 1_f32
        } else {
            (self.position.y - rem.y) / square_size
        };
        // self.pos_grid = (self.position - rem) / square_size;
        self.position = self.pos_grid * square_size;
        self.dragged = false;
        println!("self.dragged=false");
        // TODO: show distance
    }

    pub fn draw(&self, offset: &Vec2D, square_size: f32) {
        let pos = if self.dragged {
            self.position.clone()
        } else {
            self.pos_grid * square_size
        } + offset;
        // draw_texture(self.texture, pos.x, pos.y, Color::from_rgba(255, 255, 255, 255));
        let size = self.size * square_size;
        let params = DrawTextureParams {
            dest_size: Some(size.into_vec2()),
            source: None,
            rotation: 0_f32,
            flip_x: false,
            flip_y: false,
            pivot: None
        };
        self.draw_border(pos.clone(), square_size);
        draw_texture_ex(self.texture, pos.x, pos.y, Color::from_rgba(255, 255, 255, 255), params);
    }

    fn draw_border(&self, pos: Vec2D, square_size: f32) {
        let origin = pos.clone();
        let size = self.size * square_size;
        let end = pos.clone() + size.clone();
        // draw_circle_lines((end.x + origin.x) / 2_f32, (end.y + origin.y) / 2_f32, size.x / 2_f32,2_f32, Color::from_rgba(0, 0, 0, 255));
        draw_line(origin.x, origin.y, end.x, origin.y, 2_f32, Color::from_rgba(0, 0, 0, 255));
        draw_line(origin.x, origin.y, origin.x, end.y, 2_f32, Color::from_rgba(0, 0, 0, 255));
        draw_line(end.x, origin.y, end.x, end.y, 2_f32, Color::from_rgba(0, 0, 0, 255));
        draw_line(origin.x, end.y, end.x, end.y, 2_f32, Color::from_rgba(0, 0, 0, 255));
    }

    pub fn click(&mut self) -> ClickAction {
        info!("click");
        return self.click_action.clone()
    }

    pub fn clicked(&mut self, clicked: bool) -> ClickMode {
        if clicked {
            self.click_counter += 1;
            if self.click_counter >= 10 {
                self.dragged = true;
                println!("self.dragged=true");
                return ClickMode::DRAGGED;
            }
        } else {
            if self.click_counter < 10 {
                self.click_counter = 0;
                return ClickMode::CLICKED;
            } else {
                self.click_counter = 0;
                return ClickMode::NONE;
            }
        }
        return ClickMode::NONE;
    }
}

pub struct Tokenlist {
    list: Vec<Token>,
    left_mouse: MB,
    active_token_idx: Option<i32>,
}

impl Tokenlist {

    pub fn new(tokens: Option<Vec<Token>>) -> Self {
        let left_mouse = MB{ was_up: true, start: Vec2D::empty()};
        if let Some(tokens) = tokens {
            return Self{ list: tokens, left_mouse, active_token_idx: None };
        } else {
            return Self{ list: Vec::new(), left_mouse, active_token_idx: None };
        }
    }

    pub fn add(&mut self, mut token: Token, square_size: f32) {
        token.final_move(square_size);
        self.list.push(token);
    }

    pub fn click(&mut self, position: Vec2D, square_size: f32) -> Option<ClickAction> {
        // println!("x: {}, y: {}", position.0 - mouse_position().0, position.1 - mouse_position().1);

        if is_mouse_button_down(MouseButton::Left){
            if self.left_mouse.was_up {
                self.left_mouse.was_up = false;
                self.left_mouse.start = Vec2D::from(mouse_position());
                let mut idx = 0;
                for token in &mut self.list {
                    if token.in_token(position.clone(), square_size) {
                        self.active_token_idx = Some(idx);
                        token.clicked(true);
                        break;
                    }
                    idx += 1;
                }
            } else if self.active_token_idx != None {
                let drag = Vec2D::empty() - self.left_mouse.start.clone() + mouse_position();
                // println!("drag: {:?}", drag);
                let token = self.list.get_mut(self.active_token_idx.unwrap() as usize).unwrap();
                match token.clicked(true) {
                    ClickMode::DRAGGED => {
                        token.movement(drag);
                    },
                    _ => {}
                }
                self.left_mouse.start = Vec2D::from(mouse_position());
            }
        } else if !self.left_mouse.was_up && !is_mouse_button_down(MouseButton::Left){
            self.left_mouse.was_up = true;
            if let Some(index) = self.active_token_idx {
                let token = self.list.get_mut(index as usize).unwrap();
                match token.clicked(false){
                    ClickMode::CLICKED => return Some(token.click()),
                    _ => {},
                }
                token.final_move(square_size);
                self.active_token_idx = None;
            }
            self.active_token_idx = None;
        }
        None
    }

    pub fn draw_all(&self, offset: &Vec2D, square_size: f32) {
        for token in self.list.iter().rev() {
            token.draw(&offset, square_size);
        }
    }

    pub fn final_move(&mut self, square_size: f32) {
        for token in &mut self.list {
            token.final_move(square_size);
        }
    }
}