#![allow(dead_code)]
// use nannou::lyon;
use nannou::lyon::algorithms::path::math::Point;
use nannou::lyon::algorithms::path::PathSlice;
use nannou::lyon::algorithms::walk::{walk_along_path, RepeatedPattern};
use nannou::lyon::path::iterator::*;
// use nannou::lyon::path::builder::{ild, ild, PathBuilder};

// use nannou::lyon::path::iterator::*;
use nannou::prelude::*;

mod animation;
mod circle;
mod mobject;
mod path;
mod walk;
use mobject::TextMObject;
// use walk::print_distance;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    text: TextMObject,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(640, 480).view(view).build().unwrap();
    let win_rect = app.main_window().rect().pad_left(20.0);
    Model {
        text: TextMObject::new("Hello", win_rect),
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    // let path = &model.text.inner.path;
    // let mut path_points: Vec<Point> = Vec::new();
    // dots_along_path(
    //     path.as_slice(),
    //     &mut path_points,
    //     2550.0,
    //     // 0.0,
    //     (app.elapsed_frames() as f32) * 15.0,
    // );
    // path_points.iter().for_each(|p| {
    //     model.text.inner.points.push(pt2(p.x, p.y));
    // });
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let path = &model.text.inner.path;
    draw.path().fill().color(RED).events(path);
    // let mut path_points: Vec<Point> = Vec::new();

    // dots_along_path(
    //     path.as_slice(),
    //     &mut path_points,
    //     2550.0,
    //     // 0.0,
    //     app.elapsed_frames() as f32,
    //     &draw,
    // );
    // if app.elapsed_frames() == 1 {
    //     print_distance(path.iter().flattened(0.01));
    // };
    // get_distance(path.iter().flattened(0.01));

    // for p in model.text.inner.points.iter() {
    //     draw.ellipse()
    //         .radius(1.3)
    //         .x_y(p.x, p.y)
    //         .rgba(1.0, 1.0, 1.0, 1.0);
    // }
    // let seg_count = std::cmp::max(2, model.text.inner.points.len()) - 2;
    // for i in 0..seg_count {
    //     let current = model.text.inner.points[i];
    //     let next = model.text.inner.points[i + 1];
    //     draw.line()
    //         .start(current)
    //         .end(next)
    //         .rgba(1.0, 1.0, 1.0, 1.0);
    // }

    // path_points.iter().enumerate().for_each(|(i, p)| {
    //     // Lines
    //     let l = 5.0;
    //     draw.line()
    //         .start(pt2(p.x + l, p.y - l))
    //         .end(pt2(p.x - l, p.y + l))
    //         .rgb(0.7, 0.61, 0.0);
    //     // Dots
    //     let q = map_range(i, 0, path_points.len(), 0.0, 1.0);
    //     if i % 2 == 0 {
    //         draw.ellipse()
    //             .x_y(p.x, p.y)
    //             .radius(map_range(
    //                 (i as f32 * 0.05 + app.time * 4.3).sin(),
    //                 -1.0,
    //                 1.0,
    //                 3.0,
    //                 9.0,
    //             ))
    //             .hsv(q, 1.0, 0.5);
    //     }
    // });

    draw.to_frame(app, &frame).unwrap();
}
fn dots_along_path(
    path: PathSlice,
    dots: &mut Vec<Point>,
    interval: f32,
    offset: f32,
    draw: &Draw,
) {
    use std::ops::Rem;
    let dot_spacing = map_range(interval, 0.0, 1.0, 0.025, 1.0);
    // let dot_spacing = 60.0;
    let mut pattern = RepeatedPattern {
        callback: &mut |position, _tangent, _distance| {
            dots.push(position);
            draw.ellipse()
                .radius(1.3)
                .x_y(position.x, position.y)
                .rgba(1.0, 1.0, 1.0, 1.0);
            true // Return true to continue walking the path.
        },
        // Invoke the callback above at a regular interval of 3 units.
        intervals: &[dot_spacing], // 0.05],// 0.05],
        index: 0,
    };

    let tolerance = 0.01; // The path flattening tolerance.
    let start_offset = offset.rem(12.0 + dot_spacing); // Start walking at the beginning of the path.
    walk_along_path(path.iter().flattened(tolerance), start_offset, &mut pattern);
}
