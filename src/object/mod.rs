use std::cell::RefCell;
use std::rc::Rc;

use crate::animation::{Action, Animate, SetPosition, TargetAction};
use crate::draw::Draw;

use nannou;
use nannou::geom::{Point2, Vector2};

use self::circle::Circle;

pub mod circle;

pub type RefObject = Rc<RefCell<Object>>;
#[derive(Debug, PartialEq)]
pub enum Object {
    Circle(Circle),
    None,
}
impl SetPosition for Object {
    fn position(&self) -> Point2 {
        if let Object::Circle(c) = self {
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
    fn shift(&self, by: Vector2) -> TargetAction {
        TargetAction::new(
            self.clone(),
            Action::Shift {
                from: self.position(),
                by,
            },
            true,
        )
    }
    fn move_to(&self, to: Point2) -> TargetAction {
        TargetAction::new(
            self.clone(),
            Action::MoveTo {
                from: self.position(),
                to,
            },
            true,
        )
    }
    fn to_edge(&self, edge: Vector2) -> TargetAction {
        TargetAction::new(
            self.clone(),
            Action::MoveTo {
                from: self.position(),
                to: edge,
            },
            true,
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
