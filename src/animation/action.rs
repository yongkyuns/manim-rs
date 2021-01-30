use super::Interpolate;
use crate::animation::PathCompletion;
use crate::appearance::Visibility;
use crate::arena::Object;
use crate::geom::{GetPosition, Point, SetPosition, Vector};
use crate::scene::Resource;

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
    pub fn init(&mut self, object: &mut Object, resource: &Resource) {
        let position = object.position();
        match self {
            Action::Shift {
                ref mut from,
                by: _,
            } => {
                *from = position;
            }
            Action::MoveTo {
                ref mut from,
                to: _,
            } => {
                *from = position;
            }
            Action::ToEdge {
                ref mut from,
                ref mut to,
                buffer,
                direction,
            } => {
                let mut p = position;
                match direction {
                    Direction::Up => p.y = resource.edge_upper() - *buffer,
                    Direction::Down => p.y = resource.edge_lower() + *buffer,
                    Direction::Right => p.x = resource.edge_right() - *buffer,
                    Direction::Left => p.x = resource.edge_left() + *buffer,
                };
                *from = position;
                *to = p;
            }
            Action::ShowCreation => {
                object.show();
                object.set_completion(0.0);
            }
            _ => (),
        };
    }
    pub fn update(&mut self, object: &mut Object, progress: f32) {
        match self {
            Action::Shift { ref mut from, by } => {
                let ref to = *from + *by;
                let now = from.interp(to, progress);
                object.move_to(now.x, now.y);
            }
            Action::MoveTo { ref mut from, to } => {
                let now = from.interp(to, progress);
                object.move_to(now.x, now.y);
            }
            Action::ToEdge {
                ref mut from,
                ref mut to,
                buffer: _,
                direction: _,
            } => {
                let now = from.interp(to, progress);
                object.move_to(now.x, now.y);
            }
            Action::ShowCreation => {
                object.set_completion(progress);
            }
            _ => (),
        };
    }
    pub fn complete(&mut self, object: &mut Object) {
        self.update(object, 1.0);
    }
}
