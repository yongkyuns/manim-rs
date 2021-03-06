use crate::animation::PathCompletion;
use crate::appearance::{GetOpacity, Opacity, SetOpacity};
use crate::arena::Object;
use crate::consts::*;
use crate::draw::Draw;
use crate::geom;
use crate::geom::{dimension, Dimension, GetDimension, SetDimension};
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
    dimension: Dimension,
    position: geom::Point,
    path_completion: f32,
    color: Rgb,
    stroke_color: Rgb,
    opacity: Opacity,
}

impl Circle {
    fn new() -> Self {
        Circle {
            dimension: dimension(12.0, 12.0),
            position: point_at(0.0, 0.0),
            path_completion: 1.0,
            color: DEFAULT_FILL_COLOR,
            stroke_color: DEFAULT_STROKE_COLOR,
            opacity: Opacity::new(false),
        }
    }

    pub fn radius(&self) -> f32 {
        self.width() / 2.0
    }

    pub fn set_radius(&mut self, radius: f32) {
        self.set_width(radius * 2.0);
        self.set_height(radius * 2.0);
    }
}

impl Draw for Circle {
    fn draw(&self, draw: nannou::Draw) {
        if self.is_visible() {
            let mut builder = Path::builder();
            let sweep_angle = Angle::radians(PI * 2.0);
            let x_rotation = Angle::radians(0.0);
            let center: lyon::Point = point(0.0, 0.0);
            let start = point(self.radius(), 0.0);
            let radii = Vector::new(self.radius(), self.radius());

            builder.move_to(start);
            builder.arc(center, radii, sweep_angle, x_rotation);
            builder.close();

            let path = builder.build();
            let path = path.upto(self.path_completion, DEFAULT_FLATTEN_TOLERANCE);

            let color = Rgba {
                color: self.color,
                alpha: self.alpha(),
            };
            let stroke_color = Rgba {
                color: self.stroke_color,
                alpha: self.alpha(),
            };

            draw.path()
                .fill()
                .x_y(self.position.x, self.position.y)
                .color(color)
                .events(&path);
            draw.path()
                .stroke()
                .x_y(self.position.x, self.position.y)
                .color(stroke_color)
                .stroke_weight(DEFAULT_STROKE_WEIGHT)
                .events(&path);
        }
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

impl GetDimension for Circle {
    fn dimension(&self) -> &Dimension {
        GetDimension::dimension(&self.dimension)
    }
}

impl SetDimension for Circle {
    fn dimension_mut(&mut self) -> &mut Dimension {
        SetDimension::dimension_mut(&mut self.dimension)
    }
    fn set_width(&mut self, width: f32) {
        self.set_size(dimension(width, width));
    }
    fn set_height(&mut self, height: f32) {
        self.set_size(dimension(height, height));
    }
}

impl GetOpacity for Circle {
    fn opacity(&self) -> f32 {
        GetOpacity::opacity(&self.opacity)
    }
    fn is_visible(&self) -> bool {
        GetOpacity::is_visible(&self.opacity)
    }
}

impl SetOpacity for Circle {
    fn opacity_mut(&mut self) -> &mut Opacity {
        SetOpacity::opacity_mut(&mut self.opacity)
    }
}

pub fn circle() -> Object {
    Object::new(Circle::new().into())
}
