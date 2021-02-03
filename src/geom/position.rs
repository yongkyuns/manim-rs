use crate::animation::{lerp, Interpolate};
use std::ops::Add;

use nannou::lyon::math as lyon;

pub type Vector = Point;

pub fn point() -> Point {
    Point::new()
}

pub fn point_at(x: f32, y: f32) -> Point {
    Point { x, y }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl SetPosition for Point {
    fn position_mut(&mut self) -> &mut Point {
        self
    }
}

impl GetPosition for Point {
    fn position(&self) -> Point {
        *self
    }
}

impl Interpolate for Point {
    fn interp_mut(&mut self, other: &Self, progress: f32) {
        self.x = lerp(self.x, other.x, progress);
        self.y = lerp(self.y, other.y, progress);
    }
    fn interp(&self, other: &Self, progress: f32) -> Self {
        let x = lerp(self.x, other.x, progress);
        let y = lerp(self.y, other.y, progress);
        point_at(x, y)
    }
}

impl From<Point> for lyon::Point {
    fn from(p: Point) -> Self {
        lyon::Point::new(p.x, p.y)
    }
}

impl From<lyon::Point> for Point {
    fn from(p: lyon::Point) -> Self {
        point_at(p.x, p.y)
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub trait SetPosition: GetPosition {
    fn position_mut(&mut self) -> &mut Point;

    fn move_by(&mut self, x: f32, y: f32) {
        self.position_mut().x += x;
        self.position_mut().y += y;
    }

    fn move_to(&mut self, x: f32, y: f32) {
        self.position_mut().x = x;
        self.position_mut().y = y;
    }
}
pub trait GetPosition {
    fn position(&self) -> Point;
}
