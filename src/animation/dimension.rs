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
    Scale {
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
            ChangeSize::Scale {
                ref mut from,
                ref mut to,
                by,
                ..
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
            ChangeSize::Scale { from, to, .. } => {
                let w = from.width().interp(&to.width(), progress);
                let h = from.height().interp(&to.height(), progress);
                object.set_width(w);
                object.set_height(h);
            }
        }
    }
    pub fn scale_by(by: f32) -> Self {
        Self::Scale {
            from: dimension(0.0, 0.0),
            to: dimension(0.0, 0.0),
            by,
        }
    }
}
