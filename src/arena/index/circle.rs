use crate::animation::Interpolate;
use crate::animation::{Action, TargetAction};
use crate::arena::{Index, NodeIndex, Object};
use crate::object::Object as InnerObject;
use crate::scene::Resource;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CircleId(pub Index);

impl CircleId {
    pub fn set_radius(&self, radius: f32) -> TargetAction {
        let id: Index = Self::into(*self);
        TargetAction::new(
            NodeIndex(id),
            Action::CircleAction(CircleAction::SetRadius {
                from: 1.0, // This is dummy, overwritten in Action::init()
                to: radius,
            }),
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
    pub fn init(&mut self, object: &mut Object, _resource: &Resource) {
        if let InnerObject::Circle(ref c) = object.inner {
            match self {
                CircleAction::SetRadius { ref mut from, .. } => {
                    *from = c.radius();
                }
                _ => (),
            }
        }
    }
    pub fn update(&mut self, object: &mut Object, progress: f32) {
        if let InnerObject::Circle(ref mut c) = object.inner {
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
