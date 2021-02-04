// use crate::animation::Interpolate;
use crate::animation::{Action, TargetAction};
use crate::arena::{Index, NodeIndex, Object};
// use crate::object::Object as InnerObject;
use crate::scene::Resource;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextId(pub Index);

impl TextId {
    pub fn set_radius(&self, radius: f32) -> TargetAction {
        let id: Index = Self::into(*self);
        TargetAction::new(
            NodeIndex(id),
            Action::TextAction(TextAction::SetText {
                from: 1.0, // This is dummy, overwritten in Action::init()
                to: radius,
            }),
            true,
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
    SetText { from: f32, to: f32 },
    None,
}

impl TextAction {
    pub fn init(&mut self, _object: &mut Object, _resource: &Resource) {
        match self {
            TextAction::SetText { .. } => {}
            _ => (),
        }
    }
    pub fn update(&mut self, _object: &mut Object, _progress: f32) {
        match self {
            TextAction::SetText { .. } => {}
            _ => (),
        }
    }
}
