// use crate::animation::Interpolate;
use crate::animation::{Action, Interpolate, TargetAction};
use crate::arena;
use crate::arena::{Id, Index};
use crate::object::Object;
use crate::scene::Resource;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextId(pub Index);

impl TextId {
    pub fn scale_by(&self, by: f32) -> TargetAction {
        let id: Index = Self::into(*self);
        TargetAction::new(
            Id(id),
            Action::TextAction(TextAction::ScaleSize { from: 1, to: 1, by }),
        )
    }
    pub fn set_font_size(&self, to: u32) -> TargetAction {
        let id: Index = Self::into(*self);
        TargetAction::new(
            Id(id),
            Action::TextAction(TextAction::SetSize { from: 1, to }),
        )
    }
}

// Actionable is auto-implemented on `Into<Index>`
impl From<Index> for TextId {
    fn from(index: Index) -> Self {
        Self(index)
    }
}

impl From<TextId> for Index {
    fn from(id: TextId) -> Self {
        id.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TextAction {
    SetSize { from: u32, to: u32 },
    ScaleSize { from: u32, to: u32, by: f32 },
}

impl TextAction {
    pub fn init(&mut self, object: &mut arena::Object, _resource: &Resource) {
        if let Object::Text(ref text) = object.inner {
            match self {
                TextAction::SetSize { ref mut from, .. } => {
                    *from = text.font_size();
                }
                TextAction::ScaleSize {
                    ref mut from,
                    ref mut to,
                    by,
                } => {
                    *from = text.font_size();
                    *to = (text.font_size() as f32 * (*by)) as u32;
                }
            }
        }
    }
    pub fn update(&mut self, object: &mut arena::Object, progress: f32) {
        if let Object::Text(ref mut text) = object.inner {
            match self {
                TextAction::SetSize { from, to } => {
                    let now = from.interp(to, progress);
                    text.set_font_size(now);
                }
                TextAction::ScaleSize { from, to, .. } => {
                    let now = from.interp(to, progress);
                    text.set_font_size(now);
                }
            }
        }
    }
    pub fn set_font_size(&mut self, to: u32) -> Self {
        Self::SetSize { from: 1, to }
    }
    pub fn scale_by(&mut self, by: f32) -> Self {
        Self::ScaleSize { from: 1, to: 1, by }
    }
}
