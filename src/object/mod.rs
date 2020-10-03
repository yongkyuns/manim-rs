pub mod circle;

use self::circle::Circle;

use crate::animation::{Action, Animate, SetPosition};
use crate::draw::Draw;

use nannou;
use nannou::geom::{Point2, Vector2};

use std::cell::RefCell;
use std::rc::Rc;

pub type RefObject = Rc<RefCell<Object>>;
#[derive(Debug, PartialEq)]
pub enum Object {
    Circle(Circle),
    None,
}
impl SetPosition for Object {
    fn position(&self) -> Point2 {
        if let Object::Circle(c) = self {
            // println!("{:?}", c.position);
            c.position
        } else {
            Point2 { x: 0.0, y: 0.0 }
        }
    }
    fn set_position(&mut self, to: Point2) {
        match self {
            Object::Circle(c) => c.set_position(to),
            _ => (),
        }
    }
}
impl Draw for Object {
    fn draw(&self, draw: nannou::Draw) {
        match self {
            Object::Circle(c) => c.draw(draw),
            _ => (),
        }
    }
}

impl Animate for RefObject {
    // Create animation builder with given action, and return it
    // Builder allows chaining of commands for specifying animation properties
    // Builder generates Animation on drop
    // Animation::new(self.clone(), Action::Shift(by))
    fn shift(&self, by: Vector2) -> (RefObject, Action) {
        (
            self.clone(),
            Action::Shift {
                from: self.position(),
                by,
            },
        )
    }
}

impl SetPosition for RefObject {
    fn position(&self) -> Point2 {
        SetPosition::position(&*self.clone().borrow_mut())
    }
    fn set_position(&mut self, to: Point2) {
        self.borrow_mut().set_position(to);
    }
}

impl Draw for RefObject {
    fn draw(&self, draw: nannou::Draw) {
        self.borrow().draw(draw);
    }
}
