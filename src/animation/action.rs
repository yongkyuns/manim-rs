use super::{Interpolate, SetPosition};
use crate::object::RefObject;

use nannou::geom::{Point2, Vector2};

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Shift { from: Point2, by: Vector2 },
    MoveTo { from: Point2, to: Point2 },
    Scale { from: f32, to: f32 },
    Rotate { from: f32, to: f32 },
    ShowCreation,
    Write,
    FadeOut,
    Transform,
}
impl Action {
    pub fn init(&mut self, object: &RefObject) {
        match self {
            Action::Shift { from: _, by } => {
                let pos = object.position();
                *self = Action::Shift { from: pos, by: *by };
            }
            Action::MoveTo { from: _, to } => {
                let pos = object.position();
                *self = Action::MoveTo { from: pos, to: *to };
            }
            // Action::Scale { from: _, to } => {
            //     *self = Action::Scale { from: , to: *to };
            // }
            _ => (),
        }
    }
    pub fn update(&self, object: &mut RefObject, progress: f32) {
        match self {
            Action::Shift { from, by } => {
                let ref to = *from + *by;
                let now = object.position().interp(from, to, progress);
                object.set_position(now);
            }
            _ => (),
        }
    }
    pub fn complete(&self, object: &mut RefObject) {
        self.update(object, 1.0);
    }
}
