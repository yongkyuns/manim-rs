pub use nannou::geom::{self, pt2, Point2};
use nannou::math::BaseFloat;
pub use pennereq::*;

type EaseFn<S> = fn(t: S, b: S, c: S, d: S) -> S;

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub enum EaseType {
    Quad,
    QuadIn,
    QuadOut,
    Cubic,
    CubicIn,
    CubicOut,
    Quart,
    QuartIn,
    QuartOut,
    Quint,
    QuintIn,
    QuintOut,
    Sine,
    SineIn,
    SineOut,
    Expo,
    ExpoIn,
    ExpoOut,
    Circ,
    CircIn,
    CircOut,
    Elastic,
    ElasticIn,
    ElasticOut,
    Back,
    BackIn,
    BackOut,
    Bounce,
    BounceIn,
    BounceOut,
}

pub trait Tween {
    // Update current states based on initial, final target, and normalized progress.
    fn update(&mut self, initial: &Self, target: &Self, progress: f32);
}

pub struct TweenSet<T>
where
    T: Tween,
{
    pub initial: T,
    pub current: T,
    pub target: T,
}
impl<T> TweenSet<T>
where
    T: Tween + Copy,
{
    pub fn set_target(&mut self, target: T) {
        self.target = target;
        self.initial = self.current;
    }

    fn update(&mut self, progress: f32) {
        self.current.update(&self.initial, &self.target, progress);
    }
}
pub struct Tweener<T: Tween, S = geom::scalar::Default> {
    pub start_time: S,
    pub run_time: S,
    pub progress: S,
    pub tweens: Vec<TweenSet<T>>,
    pub ease_type: EaseType,
}

impl<T, S> Tweener<T, S>
where
    S: BaseFloat + From<f32> + Into<f32>,
    T: Tween + Copy,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&mut self, ease_type: EaseType, time_now: S, run_time: S) {
        self.ease_type = ease_type;
        self.start_time = time_now;
        self.progress = S::zero();
        self.run_time = run_time;
        // for pair in &mut self.tweens {
        //     pair.set_target(target);
        // }
    }

    pub fn update(&mut self, time_now: S) {
        let time_passed: S = time_now - self.start_time;
        let t = time_passed.min(self.run_time) / self.run_time;

        self.progress = self.calculate_ease(t);

        for pair in &mut self.tweens {
            pair.update(self.progress.into());
        }
    }

    pub fn clear(&mut self) {
        self.tweens = Vec::new();
    }

    pub fn register(&mut self, primitive: T) {
        self.tweens.push(TweenSet {
            initial: primitive,
            current: primitive,
            target: primitive,
        });
    }

    pub fn register_with_target(&mut self, initial: T, end: T) {
        self.tweens.push(TweenSet {
            initial: initial,
            current: initial,
            target: end,
        });
    }

    fn calculate_ease(&mut self, t: S) -> S {
        let ease_func: EaseFn<S> = match self.ease_type {
            EaseType::Quad => quad::ease_in_out,
            EaseType::QuadIn => quad::ease_in,
            EaseType::QuadOut => quad::ease_out,
            EaseType::Cubic => cubic::ease_in_out,
            EaseType::CubicIn => cubic::ease_in,
            EaseType::CubicOut => cubic::ease_out,
            EaseType::Quart => quart::ease_in_out,
            EaseType::QuartIn => quart::ease_in,
            EaseType::QuartOut => quart::ease_out,
            EaseType::Quint => quint::ease_in_out,
            EaseType::QuintIn => quint::ease_in,
            EaseType::QuintOut => quint::ease_out,
            EaseType::Sine => sine::ease_in_out,
            EaseType::SineIn => sine::ease_in,
            EaseType::SineOut => sine::ease_out,
            EaseType::Expo => expo::ease_in_out,
            EaseType::ExpoIn => expo::ease_in,
            EaseType::ExpoOut => expo::ease_out,
            EaseType::Circ => circ::ease_in_out,
            EaseType::CircIn => circ::ease_in,
            EaseType::CircOut => circ::ease_out,
            EaseType::Elastic => elastic::ease_in_out,
            EaseType::ElasticIn => elastic::ease_in,
            EaseType::ElasticOut => elastic::ease_out,
            EaseType::Back => back::ease_in_out,
            EaseType::BackIn => back::ease_in,
            EaseType::BackOut => back::ease_out,
            EaseType::Bounce => bounce::ease_in_out,
            EaseType::BounceIn => bounce::ease_in,
            EaseType::BounceOut => bounce::ease_out,
        };
        ease_func(t, S::zero(), S::one(), S::one())
    }
}

impl<T, S> Default for Tweener<T, S>
where
    S: BaseFloat,
    T: Tween,
{
    fn default() -> Self {
        let start_time = S::zero();
        let run_time = S::one();
        let progress = S::zero();
        let ease_type = EaseType::BounceOut;
        let tweens: Vec<TweenSet<T>> = Vec::new();

        Tweener {
            start_time,
            run_time,
            progress,
            tweens,
            ease_type,
        }
    }
}
