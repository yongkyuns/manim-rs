use crate::animation::{Action, TargetAction};
use crate::arena::{Index, NodeIndex};
pub trait Rotate: Into<Index> + Copy {
    fn rotate_to(&self, degree: f32) -> TargetAction {
        let id: Index = Self::into(*self);
        TargetAction::new(
            NodeIndex(id),
            Action::RotateTo {
                from: degree, // This is dummy, overwritten in Action::init()
                to: degree,
            },
            true,
        )
    }
    fn rotate_by(&self, degree: f32) -> TargetAction {
        let id: Index = Self::into(*self);
        TargetAction::new(
            NodeIndex(id),
            Action::RotateBy {
                from: degree, // This is dummy, overwritten in Action::init()
                by: degree,
            },
            true,
        )
    }
}
