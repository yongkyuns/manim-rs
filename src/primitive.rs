use crate::tween::Tween;
use nannou::geom::{self, Point2};
use nannou::math::BaseFloat;

#[derive(Debug, Clone, Copy)]
pub struct Arrow<S = geom::scalar::Default> {
    pub start: Point2<S>,
    pub end: Point2<S>,
}

impl<S> Tween for Arrow<S>
where
    S: BaseFloat,
{
    fn update(&mut self, initial: &Self, target: &Self, progress: f32) {
        let p = S::from(progress).unwrap();
        self.start = initial.start * (S::one() - p) + target.start * p;
        self.end = initial.end * (S::one() - p) + target.end * p;
    }
}
