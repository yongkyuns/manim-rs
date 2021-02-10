use crate::animation::{Action, ChangeSize, Interpolate, TargetAction};
use crate::arena;
use crate::arena::{Id, Index};
use crate::object::Object;
use crate::scene::Resource;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CircleId(pub Index);

impl CircleId {
    pub fn set_radius(&self, radius: f32) -> TargetAction {
        let id: Index = Self::into(*self);
        TargetAction::new(
            Id(id),
            // Action::CircleAction(CircleAction::SetRadius {
            //     from: 1.0, // This is dummy, overwritten in Action::init()
            //     to: radius,
            // }),
            Action::ChangeSize(ChangeSize::set_width(radius * 2.0)),
            true,
        )
    }
}

// Actionable is auto-implemented on `Into<Index>`
impl From<Index> for CircleId {
    fn from(index: Index) -> Self {
        Self(index)
    }
}

impl From<CircleId> for Index {
    fn from(id: CircleId) -> Self {
        id.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CircleAction {
    SetRadius { from: f32, to: f32 },
    None,
}

impl CircleAction {
    pub fn init(&mut self, object: &mut arena::Object, _resource: &Resource) {
        if let Object::Circle(ref c) = object.inner {
            match self {
                CircleAction::SetRadius { ref mut from, .. } => {
                    *from = c.radius();
                }
                _ => (),
            }
        }
    }
    pub fn update(&mut self, object: &mut arena::Object, progress: f32) {
        if let Object::Circle(ref mut c) = object.inner {
            match self {
                CircleAction::SetRadius { from, to } => {
                    let r = from.interp(to, progress);
                    c.set_radius(r);
                }
                _ => (),
            }
        }
    }
}
