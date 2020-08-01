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
    fn update(&mut self, progress: f32, target: &Self) {
        let p = S::from(progress).unwrap();
        self.end = self.end * (S::one() - p) + target.end * p;
        println!("{:?}", p);
    }
}
