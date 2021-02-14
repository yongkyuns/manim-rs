use crate::animation::PathCompletion;
use crate::appearance::{GetOpacity, Opacity, SetOpacity};
use crate::arena::Object;
use crate::consts::*;
use crate::draw::Draw;
use crate::geom;
use crate::geom::{dimension, Dimension, GetDimension, SetDimension};
use crate::geom::{GetOrientation, GetPosition, SetOrientation, SetPosition};
use crate::path::GetPartial;

use nannou;
use nannou::color::{Rgb, Rgba};
use nannou::lyon::math::point;
use nannou::lyon::path::Path;

#[derive(Debug, PartialEq)]
pub struct Rectangle {
    position: geom::Point,
    orientation: f32,
    dimension: Dimension,
    path_completion: f32,
    color: Rgb,
    stroke_color: Rgb,
    opacity: Opacity,
}

impl Rectangle {
    fn new() -> Self {
        Rectangle {
            dimension: dimension(30.0, 30.0),
            orientation: 0.0,
            position: geom::point(),
            path_completion: 1.0,
            color: RED_D,
            stroke_color: DEFAULT_STROKE_COLOR,
            opacity: Opacity::new(false),
        }
    }
}

impl Draw for Rectangle {
    fn draw(&self, draw: nannou::Draw) {
        if self.is_visible() {
            let mut builder = Path::builder();
            let start = point(-self.width() / 2.0, self.height() / 2.0);

            builder.move_to(start);
            builder.line_to(point(start.x + self.width(), start.y));
            builder.line_to(point(start.x + self.width(), start.y - self.height()));
            builder.line_to(point(start.x, start.y - self.height()));
            builder.line_to(point(start.x, start.y));
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

            // Draw fill first
            draw.path()
                .fill()
                .x_y(self.position.x, self.position.y)
                .z_degrees(self.orientation)
                .color(color)
                .events(&path);

            // Draw stroke on top
            draw.path()
                .stroke()
                .x_y(self.position.x, self.position.y)
                .z_degrees(self.orientation)
                .color(stroke_color)
                .stroke_weight(DEFAULT_STROKE_WEIGHT)
                .events(&path);
        }
    }
}

impl PathCompletion for Rectangle {
    fn completion(&self) -> f32 {
        self.path_completion
    }
    fn set_completion(&mut self, completion: f32) {
        self.path_completion = completion.max(0.0).min(1.0);
    }
}

impl SetPosition for Rectangle {
    fn position_mut(&mut self) -> &mut geom::Point {
        SetPosition::position_mut(&mut self.position)
    }
}

impl GetPosition for Rectangle {
    fn position(&self) -> geom::Point {
        GetPosition::position(&self.position)
    }
}

impl GetOrientation for Rectangle {
    fn orientation(&self) -> f32 {
        self.orientation
    }
}

impl SetOrientation for Rectangle {
    fn orientation_mut(&mut self) -> &mut f32 {
        &mut self.orientation
    }
}

impl GetDimension for Rectangle {
    fn dimension(&self) -> &Dimension {
        GetDimension::dimension(&self.dimension)
    }
}

impl SetDimension for Rectangle {
    fn dimension_mut(&mut self) -> &mut Dimension {
        SetDimension::dimension_mut(&mut self.dimension)
    }
}

impl GetOpacity for Rectangle {
    fn opacity(&self) -> f32 {
        GetOpacity::opacity(&self.opacity)
    }
    fn is_visible(&self) -> bool {
        GetOpacity::is_visible(&self.opacity)
    }
}

impl SetOpacity for Rectangle {
    fn opacity_mut(&mut self) -> &mut Opacity {
        SetOpacity::opacity_mut(&mut self.opacity)
    }
}

pub fn rectangle() -> Object {
    Object::new(Rectangle::new().into())
}
