use super::{Interpolate, TargetAction};
use crate::animation::PathCompletion;
use crate::appearance::Visibility;
use crate::arena::{CircleAction, Index, NodeIndex, Object, RectangleAction};
use crate::consts::*;
use crate::geom::{point, GetPosition, Point, SetPosition, Vector};
use crate::object::Object as InnerObject;
use crate::scene::Resource;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
/// Describes the action and target object.
/// Returns `TargetAction` which can change object instantly, or
/// Furuther gets converted to `Animation` which contains duration and interpolation function
pub trait Actionable {
    fn shift(&self, by: Vector) -> TargetAction;
    fn move_to(&self, to: Point) -> TargetAction;
    fn to_edge(&self, direction: Vector) -> TargetAction;
    fn show_creation(&self) -> TargetAction;
    fn scale(&self, by: f32) -> TargetAction;
}

impl<T> Actionable for T
where
    T: Into<Index> + Sized + Copy,
{
    fn shift(&self, by: Vector) -> TargetAction {
        let index: Index = T::into(*self);
        TargetAction::new(NodeIndex(index), Action::Shift { from: point(), by }, true)
    }
    fn move_to(&self, to: Point) -> TargetAction {
        let index: Index = T::into(*self);
        TargetAction::new(NodeIndex(index), Action::MoveTo { from: point(), to }, true)
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
        let index: Index = T::into(*self);
        TargetAction::new(
            NodeIndex(index),
            Action::ToEdge {
                from: point(),
                to: point(),
                buffer: MED_SMALL_BUFF,
                direction: dir_enum,
            },
            true,
        )
    }
    fn show_creation(&self) -> TargetAction {
        let index: Index = T::into(*self);
        TargetAction::new(NodeIndex(index), Action::ShowCreation, true)
    }
    fn scale(&self, by: f32) -> TargetAction {
        let index: Index = T::into(*self);
        TargetAction::new(NodeIndex(index), Action::Scale { from: 1.0, to: by }, true)
    }
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
    CircleAction(CircleAction),
    RectangleAction(RectangleAction),
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
            Action::Scale { ref mut from, .. } => {
                if let InnerObject::Circle(ref c) = object.inner {
                    *from = c.radius();
                }
            }
            Action::CircleAction(action) => {
                action.init(object, resource);
            }
            Action::RectangleAction(action) => {
                action.init(object, resource);
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
            Action::Scale { from, to } => {
                if let InnerObject::Circle(ref mut c) = object.inner {
                    let r = from.interp(to, progress);
                    c.set_radius(r);
                }
            }
            Action::CircleAction(action) => {
                action.update(object, progress);
            }
            Action::RectangleAction(action) => {
                action.update(object, progress);
            }
            _ => (),
        };
    }
    pub fn complete(&mut self, object: &mut Object) {
        self.update(object, 1.0);
    }
}
