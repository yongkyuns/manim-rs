use crate::animation::{Action, TargetAction};
use crate::arena::{Id, Index};
pub trait Dimension: Into<Index> + Copy {
    fn set_width(&self, degree: f32) -> TargetAction {
        let id: Index = Self::into(*self);
        TargetAction::new(
            Id(id),
            Action::RotateTo {
                from: degree, // This is dummy, overwritten in Action::init()
                to: degree,
            },
            true,
        )
    }
    fn set_height(&self, degree: f32) -> TargetAction {
        let id: Index = Self::into(*self);
        TargetAction::new(
            Id(id),
            Action::RotateBy {
                from: degree, // This is dummy, overwritten in Action::init()
                by: degree,
            },
            true,
        )
    }
}
