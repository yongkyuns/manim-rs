use crate::animation::PathCompletion;
use crate::appearance::Visibility;
use crate::arena::Object;
use crate::consts::*;
use crate::draw::Draw;
use crate::geom;
use crate::geom::{dimension, Dimension, GetDimension, SetDimension};
use crate::geom::{point_at, GetOrientation, GetPosition, SetOrientation, SetPosition};
use crate::path::GetPartial;

use nannou;
use nannou::color::{Rgb, Rgba};
// use nannou::lyon::math::{point, Angle, Vector};
use nannou::lyon::path::builder::PathBuilder;
use nannou::lyon::path::Path;
// use nannou::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Text {
    string: String,
    font_size: u32,
    dimension: Dimension,
    position: geom::Point,
    orientation: f32,
    path_completion: f32,
    color: Rgb,
    stroke_color: Rgb,
    alpha: f32,
    visible: bool,
}

impl Text {
    fn new(text: &str) -> Self {
        Text {
            string: String::from(text),
            font_size: 90,
            dimension: dimension(500.0, 120.0),
            position: point_at(0.0, 0.0),
            orientation: 0.0,
            path_completion: 1.0,
            color: DEFAULT_FILL_COLOR,
            stroke_color: DEFAULT_STROKE_COLOR,
            alpha: 1.0,
            visible: false,
        }
    }
    pub fn string(&self) -> &str {
        &self.string
    }
    pub fn set_text(&mut self, text: &str) {
        self.string = String::from(text);
    }
}

impl Draw for Text {
    fn draw(&self, draw: nannou::Draw) {
        if self.visible {
            let rect =
                nannou::geom::Rect::from_w_h(self.dimension.width(), self.dimension.height());
            let text = nannou::text::text(&self.string)
                .font_size(self.font_size)
                .left_justify()
                .build(rect);

            let mut builder = Path::builder();
            for e in text.path_events() {
                builder.path_event(e);
            }
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
                .fill()
                .x_y(self.position.x, self.position.y)
                .color(color)
                .events(&path);
            draw.path()
                .stroke()
                .x_y(self.position.x, self.position.y)
                .color(stroke_color)
                .stroke_weight(DEFAULT_TEXT_STROKE_WEIGHT)
                .events(&path);
        }
    }
}

impl PathCompletion for Text {
    fn completion(&self) -> f32 {
        self.path_completion
    }
    fn set_completion(&mut self, completion: f32) {
        self.path_completion = completion.max(0.0).min(1.0);
    }
}

impl SetPosition for Text {
    fn position_mut(&mut self) -> &mut geom::Point {
        SetPosition::position_mut(&mut self.position)
    }
}

impl GetPosition for Text {
    fn position(&self) -> geom::Point {
        GetPosition::position(&self.position)
    }
}

impl GetDimension for Text {
    fn dimension(&self) -> &Dimension {
        GetDimension::dimension(&self.dimension)
    }
}

impl SetDimension for Text {
    fn dimension_mut(&mut self) -> &mut Dimension {
        SetDimension::dimension_mut(&mut self.dimension)
    }
    fn set_width(&mut self, width: f32) {
        self.set_size(dimension(width, width));
    }
    fn set_height(&mut self, height: f32) {
        self.set_size(dimension(height, height));
    }
    // fn set_size(&mut self, other: Dimension) {}
}

impl GetOrientation for Text {
    fn orientation(&self) -> f32 {
        self.orientation
    }
}

impl SetOrientation for Text {
    fn orientation_mut(&mut self) -> &mut f32 {
        &mut self.orientation
    }
}

impl Visibility for Text {
    fn visible_mut(&mut self) -> &mut bool {
        &mut self.visible
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}

pub fn text(content: &str) -> Object {
    Object::new(Text::new(content).into())
}
