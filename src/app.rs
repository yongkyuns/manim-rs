use nannou::prelude::*;

use crate::scene::{self, Construct, Scene};

use nannou::lyon::algorithms::path::math::Point;
use nannou::lyon::algorithms::path::PathSlice;
use nannou::lyon::algorithms::walk::{walk_along_path, RegularPattern};
use nannou::lyon::path::iterator::*;

pub fn run() {
    nannou::app(scene).update(update).view(view).run();
}

fn scene<'a>(app: &App) -> Scene {
    app.new_window().size(640, 480).view(view).build().unwrap();
    let win_rect = app.main_window().rect();

    let mut scene = scene::scene(win_rect);
    scene.construct();
    scene
}

fn update(app: &App, scene: &mut Scene, _update: Update) {
    scene.update(app.time);
}

fn view(app: &App, scene: &Scene, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    // let win_rect = app.main_window().rect().pad_left(20.0);
    // let text = text("Hello!!")
    //     .font_size(128)
    //     .left_justify()
    //     .build(win_rect);
    // let mut builder = Path::builder();
    // for e in text.path_events() {
    //     builder.path_event(e);
    // }
    // builder.close();
    // let path = builder.build();

    // let ratio = (app.time / 3.0).min(1.0);

    // let path2 = path.upto(ratio, 0.01);

    // draw.path().stroke().color(ORANGE).events(&path2);

    // draw.ellipse().resolution(10).radius(50.0);

    // let pts = [Point::new(0.0, 0.0), Point::new(100.0, 100.0)];
    // let evts = FromPolyline::new(true, pts);

    // let mut path_points: Vec<Point> = Vec::new();
    // dots_along_path(
    //     path.as_slice(),
    //     &mut path_points,
    //     // 12.5,
    //     10000000.0,
    //     app.elapsed_frames() as f32,
    // );

    // path_points.iter().enumerate().for_each(|(i, p)| {
    //     //Lines
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

    scene.draw(draw.clone());
    // println!("{}", app.fps());
    draw.to_frame(app, &frame).unwrap();
}

fn dots_along_path(path: PathSlice, dots: &mut Vec<Point>, interval: f32, offset: f32) {
    use std::ops::Rem;
    let dot_spacing = map_range(interval, 0.0, 1.0, 0.025, 1.0);
    let mut pattern = RegularPattern {
        callback: &mut |position, _tangent, _distance| {
            dots.push(position);
            true // Return true to continue walking the path.
        },
        // Invoke the callback above at a regular interval of 3 units.
        interval: dot_spacing, // 0.05],// 0.05],
    };

    let tolerance = 0.01; // The path flattening tolerance.
    let start_offset = offset.rem(12.0 + dot_spacing); // Start walking at the beginning of the path.
    walk_along_path(path.iter().flattened(tolerance), start_offset, &mut pattern);
}
