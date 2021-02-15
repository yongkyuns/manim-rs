use crate::animation::PathCompletion;
use crate::appearance::{GetOpacity, Opacity, SetOpacity};
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
    opacity: Opacity,
}

impl Text {
    fn new(text: &str) -> Self {
        let mut text = Text {
            string: String::from(text),
            font_size: 90,
            dimension: dimension(2000.0, 2000.0),
            position: point_at(0.0, 0.0),
            orientation: 0.0,
            path_completion: 1.0,
            color: DEFAULT_FILL_COLOR,
            stroke_color: DEFAULT_STROKE_COLOR,
            opacity: Opacity::new(false),
        };
        text.update_size();
        text
    }
    pub fn string(&self) -> &str {
        &self.string
    }
    pub fn set_text(&mut self, text: &str) {
        self.string = String::from(text);
        self.update_size();
    }
    pub fn set_font_size(&mut self, size: u32) {
        self.font_size = size;
        self.update_size();
    }
    pub fn font_size(&self) -> u32 {
        self.font_size
    }
    fn update_size(&mut self) {
        let rect = nannou::geom::Rect::from_w_h(self.width(), self.height());
        let text = nannou::text::text(&self.string)
            .font_size(self.font_size)
            .left_justify()
            .build(rect);
        let bbox = text.bounding_rect();
        self.dimension = dimension(bbox.w(), bbox.h());
    }
}

impl Draw for Text {
    fn draw(&self, draw: nannou::Draw) {
        if self.is_visible() {
            let rect = nannou::geom::Rect::from_w_h(self.width(), self.height());
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
                alpha: self.alpha(),
            };
            let stroke_color = Rgba {
                color: self.stroke_color,
                alpha: self.alpha(),
            };

            // let bbox = text.bounding_rect();
            // draw.rect()
            //     .x_y(bbox.x() + self.position().x, bbox.y() + self.position().y)
            //     .z_degrees(self.orientation)
            //     .w_h(self.width(), self.height())
            //     .color(RED_D);

            // for (_glyph, rect) in text.glyphs() {
            //     draw.rect()
            //         .x_y(rect.x() + self.position.x, rect.y() + self.position.y)
            //         .wh(rect.wh())
            //         .hsla(0.5, 1.0, 0.5, 0.5);
            // }

            draw.path()
                .fill()
                .x_y(self.position.x, self.position.y)
                .z_degrees(self.orientation)
                .color(color)
                .events(&path);
            draw.path()
                .stroke()
                .x_y(self.position.x, self.position.y)
                .z_degrees(self.orientation)
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
        self.set_size(dimension(width, self.height()));
    }
    fn set_height(&mut self, height: f32) {
        self.set_size(dimension(self.width(), height));
    }
    fn set_size(&mut self, size: Dimension) {
        let scale_w = size.width() / self.width();
        let scale_h = size.height() / self.height();
        let scale = scale_w.min(scale_h);
        let font_size = ((self.font_size as f32 * scale) as u32).max(0_u32);
        self.set_font_size(font_size);
    }
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

impl GetOpacity for Text {
    fn opacity(&self) -> f32 {
        GetOpacity::opacity(&self.opacity)
    }
    fn is_visible(&self) -> bool {
        GetOpacity::is_visible(&self.opacity)
    }
}

impl SetOpacity for Text {
    fn opacity_mut(&mut self) -> &mut Opacity {
        SetOpacity::opacity_mut(&mut self.opacity)
    }
}

pub fn text(content: &str) -> Object {
    Object::new(Text::new(content).into())
}
