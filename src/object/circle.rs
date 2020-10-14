#![allow(dead_code)]
use super::{Object, RefObject};
use crate::animation::SetPosition;
use crate::consts::*;
use crate::draw::Draw;

use nannou;
// use nannou::color::named::*;
use nannou::geom::{pt2, Point2};

use std::cell::RefCell;
use std::rc::Rc;

// pub type RefCircle = Rc<RefCell<Circle>>;
#[derive(Debug, PartialEq)]
pub struct Circle {
    pub position: Point2,
    pub radius: f32,
}

impl Circle {
    fn new() -> Self {
        Circle {
            position: pt2(0.0, 0.0),
            radius: 2.0,
        }
    }
    // fn draw(&self, draw: nannou::Draw) {
    //     // call nannou drawing function here
    //     draw.ellipse()
    //         .x_y(self.position.x, self.position.y)
    //         .radius(self.radius);
    // }
}

impl Draw for Circle {
    fn draw(&self, draw: nannou::Draw) {
        // call nannou drawing function here
        draw.ellipse()
            .x_y(self.position.x, self.position.y)
            .radius(self.radius)
            .color(DEFAULT_FILL_COLOR)
            .stroke_color(DEFAULT_STROKE_COLOR)
            .stroke_weight(DEFAULT_STROKE_WEIGHT);
    }
}

// impl Animate for Circle {
//     fn shift(&mut self, by: Vector2) -> Animation {
//         Animation::new(&mut self.position, Action::Shift(by))
//     }
// }

impl SetPosition for Circle {
    fn position(&self) -> Point2 {
        self.position
    }
    fn set_position(&mut self, to: Point2) {
        self.position = to;
    }
}

// pub fn circle() -> Circle {
//     Circle::new()
// }

pub fn circle() -> RefObject {
    Rc::new(RefCell::new(Object::Circle(Circle::new())))
}
// impl Draw for RefCircle {
//     fn draw(&self) {
//         print!("Drawing within smart ptr circle")
//     }
// }
