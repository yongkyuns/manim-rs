use crate::animation::Interpolate;
use crate::arena;
use crate::geom::{dimension, Dimension, GetDimension, SetDimension};
use crate::scene::Resource;

#[derive(Debug, Clone, PartialEq)]
pub enum ChangeSize {
    SetWidth {
        from: f32,
        to: f32,
    },
    SetHeight {
        from: f32,
        to: f32,
    },
    SetDimension {
        from: Dimension,
        to: Dimension,
    },
    ScaleDimension {
        from: Dimension,
        to: Dimension,
        by: f32,
    },
}

impl ChangeSize {
    pub fn init(&mut self, object: &mut arena::Object, _resource: &Resource) {
        match self {
            ChangeSize::SetWidth { ref mut from, .. } => {
                *from = object.width();
            }
            ChangeSize::SetHeight { ref mut from, .. } => {
                *from = object.height();
            }
            ChangeSize::SetDimension {
                ref mut from,
                ref mut to,
            } => {
                *from = dimension(object.width(), object.height());
                *to = dimension(to.width(), to.height());
            }
            ChangeSize::ScaleDimension {
                ref mut from,
                ref mut to,
                by,
            } => {
                *from = dimension(object.width(), object.height());
                *to = dimension(object.width() * (*by), object.height() * (*by));
            }
        }
    }
    pub fn update(&mut self, object: &mut arena::Object, progress: f32) {
        match self {
            ChangeSize::SetWidth { from, to } => {
                let width = from.interp(to, progress);
                object.set_width(width);
            }
            ChangeSize::SetHeight { from, to } => {
                let height = from.interp(to, progress);
                object.set_height(height);
            }
            ChangeSize::SetDimension { from, to } => {
                let now = from.interp(&to, progress);
                object.set_size(now);
            }
            ChangeSize::ScaleDimension { from, to, .. } => {
                let now = from.interp(&to, progress);
                object.set_size(now);
                // object.scale_by()
            }
        }
    }
    pub fn set_width(to: f32) -> Self {
        Self::SetWidth { from: 1.0, to }
    }
    pub fn set_height(to: f32) -> Self {
        Self::SetHeight { from: 1.0, to }
    }
    pub fn scale_by(by: f32) -> Self {
        Self::ScaleDimension {
            from: dimension(1.0, 1.0),
            to: dimension(1.0, 1.0),
            by,
        }
    }
}
