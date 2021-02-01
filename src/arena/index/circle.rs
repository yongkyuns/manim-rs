use crate::animation::{Action, TargetAction};
use crate::arena::NodeIndex;
use generational_arena::Index;
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CircleId(pub Index);

impl CircleId {
    pub fn radius(&self, radius: f32) -> TargetAction {
        let id: Index = Self::into(*self);
        TargetAction::new(
            NodeIndex(id),
            Action::Scale {
                from: 1.0,
                to: radius,
            },
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

pub enum CircleAction {
    Base(Action),
}
