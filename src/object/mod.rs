use std::cell::RefCell;
use std::rc::Rc;

use crate::animation::{Action, Animate, Direction, SetPosition, TargetAction};
use crate::consts::*;
use crate::draw::Draw;

use nannou;
// use nannou::geom::{Point, Vector};
use nannou::lyon::math::{point, Point, Vector};

use self::circle::Circle;

pub mod circle;

/// Wrapper type around `Object` to retain ownership for user while
/// `Object` is being used to construct `Scene`
pub type RefObject = Rc<RefCell<Object>>;
#[derive(Debug, PartialEq)]
pub enum Object {
    Circle(Circle),
    None,
}
impl SetPosition for Object {
    fn position(&self) -> Point {
        if let Object::Circle(c) = self {
            c.position
        } else {
            point(0.0, 0.0)
        }
    }
    fn set_position(&mut self, to: Point) {
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
    fn shift(&self, by: Vector) -> TargetAction {
        TargetAction::new(
            self.clone(),
            Action::Shift {
                from: self.position(),
                by,
            },
            true,
        )
    }
    fn move_to(&self, to: Point) -> TargetAction {
        TargetAction::new(
            self.clone(),
            Action::MoveTo {
                from: self.position(),
                to,
            },
            true,
        )
    }
    fn to_edge(&self, direction: Vector) -> TargetAction {
        // Need to map direciton vector to internal enum
        // Direction vector is used to maintain consistency in API
        // Internally, enum makes it easier to compare
        let dir_enum: Direction;
        if direction == UP {
            dir_enum = Direction::Up;
        } else if direction == DOWN {
            dir_enum = Direction::Down;
        } else if direction == LEFT {
            dir_enum = Direction::Left;
        } else if direction == RIGHT {
            dir_enum = Direction::Right;
        } else {
            panic!("Invalid direction specified!! Direction must be one of UP/DOWN/LEFT/RIGHT");
        }
        TargetAction::new(
            self.clone(),
            Action::ToEdge {
                from: self.position(),
                to: self.position(), // TODO: Fix this later
                buffer: MED_SMALL_BUFF,
                direction: dir_enum,
            },
            true,
        )
    }
}

impl SetPosition for RefObject {
    fn position(&self) -> Point {
        SetPosition::position(&*self.clone().borrow_mut())
    }
    fn set_position(&mut self, to: Point) {
        self.borrow_mut().set_position(to);
    }
}

impl Draw for RefObject {
    fn draw(&self, draw: nannou::Draw) {
        self.borrow().draw(draw);
    }
}
