use crate::animation::PathCompletion;
use crate::appearance::Visibility;
use crate::arena::Object;
use crate::consts::*;
use crate::draw::Draw;
use crate::geom;
use crate::geom::{point_at, GetPosition, SetPosition};
use crate::path::GetPartial;

use nannou;
use nannou::color::{Rgb, Rgba};
use nannou::lyon::math as lyon;
use nannou::lyon::math::{point, Angle, Vector};
use nannou::lyon::path::Path;

use std::f32::consts::PI;

#[derive(Debug, PartialEq)]
pub struct Circle {
    radius: f32,
    position: geom::Point,
    path_completion: f32,
    color: Rgb,
    stroke_color: Rgb,
    alpha: f32,
    visible: bool,
}

impl Circle {
    fn new() -> Self {
        Circle {
            radius: 12.0,
            position: point_at(0.0, 0.0),
            path_completion: 1.0,
            color: DEFAULT_FILL_COLOR,
            stroke_color: DEFAULT_STROKE_COLOR,
            alpha: 1.0,
            visible: false,
        }
    }
}

impl Draw for Circle {
    fn draw(&self, draw: nannou::Draw) {
        let mut builder = Path::builder();
        let sweep_angle = Angle::radians(PI * 2.0);
        let x_rotation = Angle::radians(0.0);
        let center: lyon::Point = self.position.into();
        let start = point(self.position.x + self.radius, self.position.y);
        let radii = Vector::new(self.radius, self.radius);

        builder.move_to(start);
        builder.arc(center, radii, sweep_angle, x_rotation);

        let path = builder.build();
        let path = path.upto(self.path_completion, DEFAULT_FLATTEN_TOLERANCE);

        let color = Rgba {
            color: self.color,
            alpha: self.alpha,
        };
        let stroke_color = Rgba {
            color: self.stroke_color,
            alpha: self.alpha,
        };

        draw.path()
            .stroke()
            .color(stroke_color)
            .stroke_weight(DEFAULT_STROKE_WEIGHT)
            .events(&path);
        draw.path().fill().color(color).events(&path);
    }
}

impl PathCompletion for Circle {
    fn completion(&self) -> f32 {
        self.path_completion
    }
    fn set_completion(&mut self, completion: f32) {
        self.path_completion = completion.max(0.0).min(1.0);
    }
}

impl SetPosition for Circle {
    fn position_mut(&mut self) -> &mut geom::Point {
        SetPosition::position_mut(&mut self.position)
    }
}

impl GetPosition for Circle {
    fn position(&self) -> geom::Point {
        GetPosition::position(&self.position)
    }
}

impl Visibility for Circle {
    fn visible_mut(&mut self) -> &mut bool {
        &mut self.visible
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}

pub fn circle() -> Object {
    Object::new(Circle::new().into())
}
