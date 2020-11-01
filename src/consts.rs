use crate::ease::EaseType;
use nannou::color::Rgb;
// use nannou::geom::Vector;
use nannou::lyon::math::Vector;
use std::marker::PhantomData;

// Shorthand notation for directions
pub const ORIGIN: Vector = Vector::new(0.0, 0.0);

pub const UP: Vector = Vector::new(0.0, 1.0);
pub const DOWN: Vector = Vector::new(0.0, -1.0);
pub const RIGHT: Vector = Vector::new(1.0, 0.0);
pub const LEFT: Vector = Vector::new(-1.0, 0.0);

pub const UL: Vector = Vector::new(-1.0, 1.0);
pub const UR: Vector = Vector::new(1.0, 1.0);
pub const DL: Vector = Vector::new(-1.0, -1.0);
pub const DR: Vector = Vector::new(1.0, -1.0);

pub const X_AXIS: Vector = Vector::new(1.0, 0.0);
pub const Y_AXIS: Vector = Vector::new(0.0, 1.0);

pub const SMALL_BUFF: f32 = 0.1;
pub const MED_SMALL_BUFF: f32 = 0.25;
pub const LARGE_BUFF: f32 = 0.5;

pub const DEFAULT_STROKE_WEIGHT: f32 = 1.0;

pub const DEFAULT_FILL_COLOR: Rgb = Rgb {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
    standard: PhantomData,
};
pub const DEFAULT_STROKE_COLOR: Rgb = Rgb {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
    standard: PhantomData,
};

// Short-hand notation for rate-function
pub const LINEAR: EaseType = EaseType::Linear;
pub const QUAD: EaseType = EaseType::Quad;
pub const QUAD_IN: EaseType = EaseType::QuadIn;
pub const QUAD_OUT: EaseType = EaseType::QuadOut;
pub const CUBIC: EaseType = EaseType::Cubic;
pub const CUBIC_IN: EaseType = EaseType::CubicIn;
pub const CUBIC_OUT: EaseType = EaseType::CubicOut;
pub const QUART: EaseType = EaseType::Quart;
pub const QUART_IN: EaseType = EaseType::QuartIn;
pub const QUART_OUT: EaseType = EaseType::QuartOut;
pub const QUINT: EaseType = EaseType::Quint;
pub const QUINT_IN: EaseType = EaseType::QuintIn;
pub const QUINT_OUT: EaseType = EaseType::QuintOut;
pub const SINE: EaseType = EaseType::Sine;
pub const SINE_IN: EaseType = EaseType::SineIn;
pub const SINE_OUT: EaseType = EaseType::SineOut;
pub const EXPO: EaseType = EaseType::Expo;
pub const EXPO_IN: EaseType = EaseType::ExpoIn;
pub const EXPO_OUT: EaseType = EaseType::ExpoOut;
pub const CIRC: EaseType = EaseType::Circ;
pub const CIRC_IN: EaseType = EaseType::CircIn;
pub const CIRC_OUT: EaseType = EaseType::CircOut;
pub const ELASTIC: EaseType = EaseType::Elastic;
pub const ELASTIC_IN: EaseType = EaseType::ElasticIn;
pub const ELASTIC_OUT: EaseType = EaseType::ElasticOut;
pub const BACK: EaseType = EaseType::Back;
pub const BACK_IN: EaseType = EaseType::BackIn;
pub const BACK_OUT: EaseType = EaseType::BackOut;
pub const BOUNCE: EaseType = EaseType::Bounce;
pub const BOUNCE_IN: EaseType = EaseType::BounceIn;
pub const BOUNCE_OUT: EaseType = EaseType::BounceOut;
