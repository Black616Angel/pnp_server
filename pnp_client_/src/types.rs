use std::{ops::{Add, AddAssign, Sub, SubAssign, Div, Rem, Mul}, cmp::Ordering};

use macroquad::prelude::Vec2;
use macroquad::math::vec2;

pub struct MB {
    pub was_up: bool,
    pub start: Vec2D,
}

#[derive(Clone, Copy, Debug)]
pub struct Vec2D {
    pub x: f32,
    pub y: f32,
}
impl Vec2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self{x,y}
    }
    pub fn empty() -> Self {
        Self{x:0_f32, y:0_f32}
    }
    
    pub fn into_vec2(&self) -> Vec2 {
        vec2(self.x.clone(), self.y.clone())
    }
}
impl Add for Vec2D {
    type Output = Vec2D;
    fn add(self, rhs: Vec2D) -> Self::Output  {
        Vec2D{ x: self.x + rhs.x, y: self.y + rhs.y }
    }
}
impl Add<&Vec2D> for Vec2D {
    type Output = Vec2D;
    fn add(self, rhs: &Vec2D) -> Self::Output  {
        Vec2D{ x: self.x + rhs.x, y: self.y + rhs.y }
    }
}
impl Add<f32> for Vec2D {
    type Output = Vec2D;
    fn add(self, rhs: f32) -> Self::Output  {
        Vec2D{ x: self.x + rhs, y: self.y + rhs }
    }
}
impl Add<(f32,f32)> for Vec2D {
    type Output = Vec2D;
    fn add(self, rhs: (f32,f32)) -> Self::Output  {
        Vec2D{ x: self.x + rhs.0, y: self.y + rhs.1 }
    }
}
impl AddAssign for Vec2D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2D {
    type Output = Vec2D;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2D{ x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Sub<(f32,f32)> for Vec2D {
    type Output = Vec2D;

    fn sub(self, rhs: (f32,f32)) -> Self::Output {
        Vec2D{ x: self.x - rhs.0, y: self.y - rhs.1 }
    }
}

impl Sub<f32> for Vec2D {
    type Output = Vec2D;

    fn sub(self, rhs: f32) -> Self::Output {
        Vec2D{ x: self.x - rhs, y: self.y - rhs }
    }
}

impl SubAssign for Vec2D {

    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Div for Vec2D {
    type Output = Vec2D;

    fn div(self, rhs: Self) -> Self::Output {
        Vec2D{ x: self.x / rhs.x, y: self.y / rhs.y }
    }
}

impl Div<f32> for Vec2D {
    type Output = Vec2D;

    fn div(self, rhs: f32) -> Self::Output {
        Vec2D{ x: self.x / rhs, y: self.y / rhs }
    }
}

impl Mul for Vec2D {
    type Output = Vec2D;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec2D{ x: self.x * rhs.x, y: self.y * rhs.y }
    }
}

impl Mul<f32> for Vec2D {
    type Output = Vec2D;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec2D{ x: self.x * rhs, y: self.y * rhs }
    }
}

impl PartialOrd for Vec2D {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.x < other.x && self.y < other.y {
            return Some(Ordering::Less)
        } else if self.x == other.x && self.y == other.y {
            return Some(Ordering::Equal)
        } else if self.x <= other.x && self.y <= other.y {
            return Some(Ordering::Less)
        } else if self.x > other.x && self.y > other.y {
            return Some(Ordering::Greater)
        } else if self.x >= other.x && self.y >= other.y {
            return Some(Ordering::Greater)
        } else {
            return None
        }
    }

    fn lt(&self, other: &Self) -> bool {
        if self.x < other.x && self.y < other.y {
            return true;
        }
        return false;
    }

    fn le(&self, other: &Self) -> bool {
        if self.x <= other.x && self.y <= other.y {
            return true
        }
        return false;
    }

    fn gt(&self, other: &Self) -> bool {
        if self.x >= other.x && self.y >= other.y {
            return true
        }
        return false;
    }

    fn ge(&self, other: &Self) -> bool {
        if self.x >= other.x && self.y >= other.y {
            return true
        }
        return false;
    }
}

impl PartialEq for Vec2D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Rem for Vec2D {
    type Output = Vec2D;

    fn rem(self, rhs: Self) -> Self::Output {
        Self{ x: self.x % rhs.x , y: self.y % rhs.y }
    }
}

impl Rem<f32> for Vec2D {
    type Output = Vec2D;

    fn rem(self, rhs: f32) -> Self::Output {
        Self{ x: self.x % rhs , y: self.y % rhs }
    }
}

impl From<(f32,f32)> for Vec2D {
    fn from(point: (f32,f32)) -> Self {
        Self{x: point.0, y: point.1}
    }
}