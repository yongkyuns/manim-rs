use crate::animation::PathCompletion;
use crate::appearance::Visibility;
use crate::arena::Object;
use crate::consts::*;
use crate::draw::Draw;
use crate::geom;
use crate::geom::{GetPosition, SetPosition};
use crate::path::GetPartial;

use nannou;
use nannou::color::{Rgb, Rgba};
use nannou::lyon::math::point;
use nannou::lyon::path::Path;

#[derive(Debug, PartialEq)]
pub struct Rectangle {
    position: geom::Point,
    width: f32,
    height: f32,
    path_completion: f32,
    color: Rgb,
    stroke_color: Rgb,
    alpha: f32,
    visible: bool,
}

impl Rectangle {
    fn new() -> Self {
        Rectangle {
            width: 30.0,
            height: 30.0,
            position: geom::point(),
            path_completion: 1.0,
            color: RED_D,
            stroke_color: DEFAULT_STROKE_COLOR,
            alpha: 1.0,
            visible: false,
        }
    }
}

impl Draw for Rectangle {
    fn draw(&self, draw: nannou::Draw) {
        if self.visible {
            let mut builder = Path::builder();
            let start = point(
                self.position.x - self.width / 2.0,
                self.position.y - self.height / 2.0,
            );
            builder.move_to(start);
            builder.line_to(point(start.x + self.width, start.y));
            builder.line_to(point(start.x + self.width, start.y - self.height));
            builder.line_to(point(start.x, start.y - self.height));
            builder.line_to(point(start.x, start.y));
            builder.close();
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

impl Visibility for Rectangle {
    fn visible_mut(&mut self) -> &mut bool {
        &mut self.visible
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}

pub fn rectangle() -> Object {
    Object::new(Rectangle::new().into())
}
