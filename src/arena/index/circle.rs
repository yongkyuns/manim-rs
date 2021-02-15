use crate::animation::{Action, ChangeSize, Interpolate, TargetAction};
use crate::arena;
use crate::arena::{Id, Index};
use crate::geom::dimension;
use crate::object::Object;
use crate::scene::Resource;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CircleId(pub Index);

impl CircleId {
    pub fn scale_by(&self, by: f32) -> TargetAction {
        let id: Index = Self::into(*self);
        TargetAction::new(
            Id(id),
            Action::ChangeSize(ChangeSize::ScaleDimension {
                from: dimension(1.0, 1.0), // This is dummy, overwritten in Action::init()
                to: dimension(1.0, 1.0),
                by,
            }),
        )
    }
    pub fn set_radius(&self, to: f32) -> TargetAction {
        let size = to * 2.0;
        let id: Index = Self::into(*self);
        TargetAction::new(
            Id(id),
            Action::ChangeSize(ChangeSize::SetDimension {
                from: dimension(size, size),
                to: dimension(size, size),
            }),
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
