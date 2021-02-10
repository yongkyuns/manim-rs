use crate::ease::EaseType;
use crate::geom::Vector;
use nannou::color::Rgb;
// use nannou::lyon::math::Vector;
use std::marker::PhantomData;

pub const DEFAULT_FLATTEN_TOLERANCE: f32 = 0.01;
pub const DEFAULT_RUNTIME: f32 = 1.0;

// Shorthand notation for directions
// pub const ORIGIN: Vector = Vector::new(0.0, 0.0);
pub const ORIGIN: Vector = Vector { x: 0.0, y: 0.0 };

pub const UP: Vector = Vector { x: 0.0, y: 1.0 };
pub const DOWN: Vector = Vector { x: 0.0, y: -1.0 };
pub const RIGHT: Vector = Vector { x: 1.0, y: 0.0 };
pub const LEFT: Vector = Vector { x: -1.0, y: 0.0 };

pub const UL: Vector = Vector { x: -1.0, y: 1.0 };
pub const UR: Vector = Vector { x: 1.0, y: 1.0 };
pub const DL: Vector = Vector { x: -1.0, y: -1.0 };
pub const DR: Vector = Vector { x: 1.0, y: -1.0 };

pub const X_AXIS: Vector = Vector { x: 1.0, y: 0.0 };
pub const Y_AXIS: Vector = Vector { x: 0.0, y: 1.0 };

pub const SMALL_BUFF: f32 = 0.1;
pub const MED_SMALL_BUFF: f32 = 0.25;
pub const LARGE_BUFF: f32 = 0.5;

pub const DEFAULT_STROKE_WEIGHT: f32 = 3.0;
pub const DEFAULT_TEXT_STROKE_WEIGHT: f32 = 3.0;

pub const DEFAULT_FILL_COLOR: Rgb = RED_D;
// pub const DEFAULT_FILL_COLOR: Rgb = Rgb {
//     red: 0.0,
//     green: 0.0,
//     blue: 0.0,
//     standard: PhantomData,
// };
pub const DEFAULT_STROKE_COLOR: Rgb = Rgb {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
    standard: PhantomData,
};

pub const DARK_BLUE: Rgb = Rgb {
    red: 35.0 / 255.0,
    green: 17.0 / 255.0,
    blue: 142.0 / 255.0,
    standard: PhantomData,
};
pub const DARK_BROWN: Rgb = Rgb {
    red: 139.0 / 255.0,
    green: 69.0 / 255.0,
    blue: 19.0 / 255.0,
    standard: PhantomData,
};
pub const LIGHT_BROWN: Rgb = Rgb {
    red: 205.0 / 255.0,
    green: 133.0 / 255.0,
    blue: 63.0 / 255.0,
    standard: PhantomData,
};
pub const BLUE_E: Rgb = Rgb {
    red: 28.0 / 255.0,
    green: 117.0 / 255.0,
    blue: 138.0 / 255.0,
    standard: PhantomData,
};
pub const BLUE_D: Rgb = Rgb {
    red: 41.0 / 255.0,
    green: 171.0 / 255.0,
    blue: 202.0 / 255.0,
    standard: PhantomData,
};
pub const BLUE_C: Rgb = Rgb {
    red: 88.0 / 255.0,
    green: 196.0 / 255.0,
    blue: 221.0 / 255.0,
    standard: PhantomData,
};
pub const BLUE_B: Rgb = Rgb {
    red: 156.0 / 255.0,
    green: 220.0 / 255.0,
    blue: 235.0 / 255.0,
    standard: PhantomData,
};
pub const BLUE_A: Rgb = Rgb {
    red: 199.0 / 255.0,
    green: 233.0 / 255.0,
    blue: 241.0 / 255.0,
    standard: PhantomData,
};
pub const TEAL_E: Rgb = Rgb {
    red: 73.0 / 255.0,
    green: 168.0 / 255.0,
    blue: 143.0 / 255.0,
    standard: PhantomData,
};
pub const TEAL_D: Rgb = Rgb {
    red: 85.0 / 255.0,
    green: 193.0 / 255.0,
    blue: 167.0 / 255.0,
    standard: PhantomData,
};
pub const TEAL_C: Rgb = Rgb {
    red: 92.0 / 255.0,
    green: 208.0 / 255.0,
    blue: 179.0 / 255.0,
    standard: PhantomData,
};
pub const TEAL_B: Rgb = Rgb {
    red: 118.0 / 255.0,
    green: 221.0 / 255.0,
    blue: 192.0 / 255.0,
    standard: PhantomData,
};
pub const TEAL_A: Rgb = Rgb {
    red: 172.0 / 255.0,
    green: 234.0 / 255.0,
    blue: 215.0 / 255.0,
    standard: PhantomData,
};
pub const GREEN_E: Rgb = Rgb {
    red: 105.0 / 255.0,
    green: 156.0 / 255.0,
    blue: 82.0 / 255.0,
    standard: PhantomData,
};
pub const GREEN_D: Rgb = Rgb {
    red: 119.0 / 255.0,
    green: 176.0 / 255.0,
    blue: 93.0 / 255.0,
    standard: PhantomData,
};
pub const GREEN_C: Rgb = Rgb {
    red: 131.0 / 255.0,
    green: 193.0 / 255.0,
    blue: 103.0 / 255.0,
    standard: PhantomData,
};
pub const GREEN_B: Rgb = Rgb {
    red: 166.0 / 255.0,
    green: 207.0 / 255.0,
    blue: 140.0 / 255.0,
    standard: PhantomData,
};
pub const GREEN_A: Rgb = Rgb {
    red: 201.0 / 255.0,
    green: 226.0 / 255.0,
    blue: 174.0 / 255.0,
    standard: PhantomData,
};
pub const YELLOW_E: Rgb = Rgb {
    red: 232.0 / 255.0,
    green: 193.0 / 255.0,
    blue: 28.0 / 255.0,
    standard: PhantomData,
};
pub const YELLOW_D: Rgb = Rgb {
    red: 244.0 / 255.0,
    green: 211.0 / 255.0,
    blue: 69.0 / 255.0,
    standard: PhantomData,
};
pub const YELLOW_C: Rgb = Rgb {
    red: 255.0 / 255.0,
    green: 255.0 / 255.0,
    blue: 0.0 / 255.0,
    standard: PhantomData,
};
pub const YELLOW_B: Rgb = Rgb {
    red: 255.0 / 255.0,
    green: 234.0 / 255.0,
    blue: 148.0 / 255.0,
    standard: PhantomData,
};
pub const YELLOW_A: Rgb = Rgb {
    red: 255.0 / 255.0,
    green: 241.0 / 255.0,
    blue: 182.0 / 255.0,
    standard: PhantomData,
};
pub const GOLD_E: Rgb = Rgb {
    red: 199.0 / 255.0,
    green: 141.0 / 255.0,
    blue: 70.0 / 255.0,
    standard: PhantomData,
};
pub const GOLD_D: Rgb = Rgb {
    red: 225.0 / 255.0,
    green: 161.0 / 255.0,
    blue: 88.0 / 255.0,
    standard: PhantomData,
};
pub const GOLD_C: Rgb = Rgb {
    red: 240.0 / 255.0,
    green: 172.0 / 255.0,
    blue: 95.0 / 255.0,
    standard: PhantomData,
};
pub const GOLD_B: Rgb = Rgb {
    red: 249.0 / 255.0,
    green: 183.0 / 255.0,
    blue: 117.0 / 255.0,
    standard: PhantomData,
};
pub const GOLD_A: Rgb = Rgb {
    red: 247.0 / 255.0,
    green: 199.0 / 255.0,
    blue: 151.0 / 255.0,
    standard: PhantomData,
};
pub const RED_E: Rgb = Rgb {
    red: 207.0 / 255.0,
    green: 80.0 / 255.0,
    blue: 68.0 / 255.0,
    standard: PhantomData,
};
pub const RED_D: Rgb = Rgb {
    red: 230.0 / 255.0,
    green: 90.0 / 255.0,
    blue: 76.0 / 255.0,
    standard: PhantomData,
};
pub const RED_C: Rgb = Rgb {
    red: 252.0 / 255.0,
    green: 98.0 / 255.0,
    blue: 85.0 / 255.0,
    standard: PhantomData,
};
pub const RED_B: Rgb = Rgb {
    red: 255.0 / 255.0,
    green: 128.0 / 255.0,
    blue: 128.0 / 255.0,
    standard: PhantomData,
};
pub const RED_A: Rgb = Rgb {
    red: 247.0 / 255.0,
    green: 161.0 / 255.0,
    blue: 163.0 / 255.0,
    standard: PhantomData,
};
pub const MAROON_E: Rgb = Rgb {
    red: 148.0 / 255.0,
    green: 66.0 / 255.0,
    blue: 79.0 / 255.0,
    standard: PhantomData,
};
pub const MAROON_D: Rgb = Rgb {
    red: 162.0 / 255.0,
    green: 77.0 / 255.0,
    blue: 97.0 / 255.0,
    standard: PhantomData,
};
pub const MAROON_C: Rgb = Rgb {
    red: 197.0 / 255.0,
    green: 95.0 / 255.0,
    blue: 115.0 / 255.0,
    standard: PhantomData,
};
pub const MAROON_B: Rgb = Rgb {
    red: 236.0 / 255.0,
    green: 147.0 / 255.0,
    blue: 171.0 / 255.0,
    standard: PhantomData,
};
pub const MAROON_A: Rgb = Rgb {
    red: 236.0 / 255.0,
    green: 171.0 / 255.0,
    blue: 193.0 / 255.0,
    standard: PhantomData,
};
pub const PURPLE_E: Rgb = Rgb {
    red: 100.0 / 255.0,
    green: 65.0 / 255.0,
    blue: 114.0 / 255.0,
    standard: PhantomData,
};
pub const PURPLE_D: Rgb = Rgb {
    red: 113.0 / 255.0,
    green: 85.0 / 255.0,
    blue: 130.0 / 255.0,
    standard: PhantomData,
};
pub const PURPLE_C: Rgb = Rgb {
    red: 154.0 / 255.0,
    green: 114.0 / 255.0,
    blue: 172.0 / 255.0,
    standard: PhantomData,
};
pub const PURPLE_B: Rgb = Rgb {
    red: 177.0 / 255.0,
    green: 137.0 / 255.0,
    blue: 198.0 / 255.0,
    standard: PhantomData,
};
pub const PURPLE_A: Rgb = Rgb {
    red: 202.0 / 255.0,
    green: 163.0 / 255.0,
    blue: 232.0 / 255.0,
    standard: PhantomData,
};
pub const WHITE: Rgb = Rgb {
    red: 255.0 / 255.0,
    green: 255.0 / 255.0,
    blue: 255.0 / 255.0,
    standard: PhantomData,
};
pub const BLACK: Rgb = Rgb {
    red: 0.0 / 255.0,
    green: 0.0 / 255.0,
    blue: 0.0 / 255.0,
    standard: PhantomData,
};
pub const LIGHT_GRAY: Rgb = Rgb {
    red: 187.0 / 255.0,
    green: 187.0 / 255.0,
    blue: 187.0 / 255.0,
    standard: PhantomData,
};
pub const LIGHT_GREY: Rgb = Rgb {
    red: 187.0 / 255.0,
    green: 187.0 / 255.0,
    blue: 187.0 / 255.0,
    standard: PhantomData,
};
pub const GRAY: Rgb = Rgb {
    red: 136.0 / 255.0,
    green: 136.0 / 255.0,
    blue: 136.0 / 255.0,
    standard: PhantomData,
};
pub const GREY: Rgb = Rgb {
    red: 136.0 / 255.0,
    green: 136.0 / 255.0,
    blue: 136.0 / 255.0,
    standard: PhantomData,
};
pub const DARK_GREY: Rgb = Rgb {
    red: 68.0 / 255.0,
    green: 68.0 / 255.0,
    blue: 68.0 / 255.0,
    standard: PhantomData,
};
pub const DARK_GRAY: Rgb = Rgb {
    red: 68.0 / 255.0,
    green: 68.0 / 255.0,
    blue: 68.0 / 255.0,
    standard: PhantomData,
};
pub const DARKER_GREY: Rgb = Rgb {
    red: 34.0 / 255.0,
    green: 34.0 / 255.0,
    blue: 34.0 / 255.0,
    standard: PhantomData,
};
pub const DARKER_GRAY: Rgb = Rgb {
    red: 34.0 / 255.0,
    green: 34.0 / 255.0,
    blue: 34.0 / 255.0,
    standard: PhantomData,
};
pub const GREY_BROWN: Rgb = Rgb {
    red: 115.0 / 255.0,
    green: 99.0 / 255.0,
    blue: 87.0 / 255.0,
    standard: PhantomData,
};
pub const PINK: Rgb = Rgb {
    red: 209.0 / 255.0,
    green: 71.0 / 255.0,
    blue: 189.0 / 255.0,
    standard: PhantomData,
};
pub const LIGHT_PINK: Rgb = Rgb {
    red: 220.0 / 255.0,
    green: 117.0 / 255.0,
    blue: 205.0 / 255.0,
    standard: PhantomData,
};
pub const GREEN_SCREEN: Rgb = Rgb {
    red: 0.0 / 255.0,
    green: 255.0 / 255.0,
    blue: 0.0 / 255.0,
    standard: PhantomData,
};
pub const ORANGE: Rgb = Rgb {
    red: 255.0 / 255.0,
    green: 134.0 / 255.0,
    blue: 47.0 / 255.0,
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
