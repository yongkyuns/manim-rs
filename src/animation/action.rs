use super::{Interpolate, SetPosition};
use crate::object::RefObject;
use crate::scene::Resource;

use nannou::lyon::math::{point, Point, Vector};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Shift {
        from: Point,
        by: Vector,
    },
    MoveTo {
        from: Point,
        to: Point,
    },
    ToEdge {
        from: Point,
        to: Point,
        buffer: f32,
        direction: Direction,
    },
    NextTo {
        from: Point,
        to: Point,
    },
    Scale {
        from: f32,
        to: f32,
    },
    Rotate {
        from: f32,
        to: f32,
    },
    ShowCreation,
    Write,
    FadeIn,
    FadeOut,
    Transform,
}
impl Action {
    pub fn init(&mut self, object: &RefObject, resource: &Resource) {
        let pos = object.position();
        match self {
            Action::Shift { from: _, by } => {
                *self = Action::Shift { from: pos, by: *by };
            }
            Action::MoveTo { from: _, to } => {
                *self = Action::MoveTo { from: pos, to: *to };
            }
            Action::ToEdge {
                from: _,
                to: _,
                buffer,
                direction,
            } => {
                let to_y = match direction {
                    Direction::Up => resource.edge_upper() - *buffer,
                    Direction::Down => resource.edge_lower() + *buffer,
                    _ => pos.y,
                };
                let to_x = match direction {
                    Direction::Left => resource.edge_left() + *buffer,
                    Direction::Right => resource.edge_right() - *buffer,
                    _ => pos.x,
                };
                *self = Action::ToEdge {
                    from: pos,
                    to: point(to_x, to_y),
                    buffer: *buffer,
                    direction: *direction,
                };
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
                let now = from.interp(to, progress);
                object.set_position(now);
            }
            Action::MoveTo { from, to } => {
                let now = from.interp(to, progress);
                object.set_position(now);
            }
            Action::ToEdge {
                from,
                to,
                buffer: _,
                direction: _,
            } => {
                let now = from.interp(to, progress);
                object.set_position(now);
            }
            // Action::ShowCreation=>{

            // }
            _ => (),
        }
    }
    pub fn complete(&self, object: &mut RefObject) {
        self.update(object, 1.0);
    }
}
