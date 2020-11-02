#![allow(dead_code)]
use super::{Object, RefObject};
use crate::animation::{PathCompletion, SetPosition};
use crate::consts::*;
use crate::draw::Draw;
use crate::path::GetPartial;

use nannou;
// use nannou::color::named::*;
use nannou::lyon::math::{point, Angle, Point, Vector};
use nannou::lyon::path::Path;

use std::cell::RefCell;
use std::f32::consts::PI;
use std::rc::Rc;

// pub type RefCircle = Rc<RefCell<Circle>>;
#[derive(Debug, PartialEq)]
pub struct Circle {
    pub position: Point,
    pub radius: f32,
    pub path_completion: f32,
}

impl Circle {
    fn new() -> Self {
        Circle {
            position: point(0.0, 0.0),
            radius: 12.0,
            path_completion: 1.0,
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
        // draw.ellipse()
        //     .x_y(self.position.x, self.position.y)
        //     .radius(self.radius)
        //     .color(DEFAULT_FILL_COLOR)
        //     .stroke_color(DEFAULT_STROKE_COLOR)
        //     .stroke_weight(DEFAULT_STROKE_WEIGHT);

        let mut builder = Path::builder();
        let sweep_angle = Angle::radians(PI * 2.0);
        let x_rotation = Angle::radians(0.0);
        let center = self.position;
        let start = point(self.position.x + self.radius, self.position.y);
        let radii = Vector::new(self.radius, self.radius);
        builder.move_to(start);
        builder.arc(center, radii, sweep_angle, x_rotation);
        let path = builder.build();
        let path = path.upto(self.path_completion, DEFAULT_FLATTEN_TOLERANCE);

        draw.path()
            .stroke()
            .color(DEFAULT_STROKE_COLOR)
            .stroke_weight(DEFAULT_STROKE_WEIGHT)
            .events(&path);
        draw.path().fill().color(DEFAULT_FILL_COLOR).events(&path);
    }
}

// impl Animate for Circle {
//     fn shift(&mut self, by: Vector2) -> Animation {
//         Animation::new(&mut self.position, Action::Shift(by))
//     }
// }

impl PathCompletion for Circle {
    fn completion(&self) -> f32 {
        self.path_completion
    }
    fn set_completion(&mut self, completion: f32) {
        self.path_completion = completion.max(0.0).min(1.0);
    }
}

impl SetPosition for Circle {
    fn position(&self) -> Point {
        self.position
    }
    fn set_position(&mut self, to: Point) {
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
