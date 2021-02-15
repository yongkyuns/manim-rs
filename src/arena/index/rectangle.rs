use crate::animation::{Action, ChangeSize, TargetAction};
use crate::arena;
use crate::arena::{Id, Index, Rotate};
use crate::geom::{dimension, GetDimension, SetDimension};
use crate::object::Object;
use crate::scene::Resource;
// use crate::{animation::Interpolate, geom::SetOrientation};
use crate::animation::Interpolate;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RectangleId(pub Index);

impl RectangleId {
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
}

impl From<Index> for RectangleId {
    fn from(index: Index) -> Self {
        Self(index)
    }
}

impl From<RectangleId> for Index {
    fn from(id: RectangleId) -> Self {
        id.0
    }
}

impl Rotate for RectangleId {}

#[derive(Debug, Clone, PartialEq)]
pub enum RectangleAction {
    SetWidth { from: f32, to: f32 },
    SetHeight { from: f32, to: f32 },
    Scale { from: (f32, f32), to: (f32, f32) },
    // RotateTo { from: f32, to: f32 },
    // RotateBy { from: f32, by: f32 },
}

impl RectangleAction {
    pub fn init(&mut self, object: &mut arena::Object, _resource: &Resource) {
        if let Object::Rectangle(ref r) = object.inner {
            match self {
                RectangleAction::SetWidth { ref mut from, .. } => {
                    *from = r.width();
                }
                RectangleAction::SetHeight { ref mut from, .. } => {
                    *from = r.height();
                }
                RectangleAction::Scale { ref mut from, .. } => *from = (r.width(), r.height()),
                // RectangleAction::RotateTo { ref mut from, .. } => {
                //     *from = r.orientation();
                // }
                // RectangleAction::RotateBy { ref mut from, .. } => {
                //     *from = r.orientation();
                // }
            }
        }
    }
    pub fn update(&mut self, object: &mut arena::Object, progress: f32) {
        if let Object::Rectangle(ref mut r) = object.inner {
            match self {
                RectangleAction::SetWidth { from, to } => {
                    let width = from.interp(to, progress);
                    r.set_width(width);
                }
                RectangleAction::SetHeight { from, to } => {
                    let height = from.interp(to, progress);
                    r.set_height(height);
                }
                RectangleAction::Scale { from, to } => {
                    let w = from.0.interp(&to.0, progress);
                    let h = from.1.interp(&to.1, progress);
                    r.set_width(w);
                    r.set_height(h);
                } // RectangleAction::RotateTo { from, to } => {
                  //     let deg = from.interp(to, progress);
                  //     r.rotate_to(deg);
                  // }
                  // RectangleAction::RotateBy { from, by } => {
                  //     let ref to = *from + *by;
                  //     let deg = from.interp(to, progress);
                  //     r.rotate_to(deg);
                  // }
            }
        }
    }
}
