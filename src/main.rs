extern crate cgmath;
extern crate pennereq;

pub mod primitive;
pub mod tween;

use nannou::prelude::*;
use primitive::Arrow;
use tween::{EaseType, Tweener};

struct Model {
    tweener: Tweener<Arrow>,
}

fn model(app: &App) -> Model {
    app.new_window().size(512, 512).view(view).build().unwrap();

    let r = app.window_rect();
    let mut tweener = Tweener::new();

    let start = r.xy();
    let end = start + pt2(100.0, 100.0);
    tweener.register(Arrow { start, end });
    Model { tweener }
}

fn main() {
    nannou::app(model).update(update).run();
}

fn update(app: &App, model: &mut Model, _update: Update) {
    if app.mouse.buttons.left().is_down() {
        let start = model.tweener.pairs[0].initial.start;
        let end = app.mouse.position();
        model
            .tweener
            .start(EaseType::BounceOut, Arrow { start, end }, app.time, 13.0);
    }
    model.tweener.update(app.time);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    for tween_pair in &model.tweener.pairs {
        let arrow = &tween_pair.initial;
        if arrow.start != arrow.end {
            draw.arrow().weight(5.0).points(arrow.start, arrow.end);
        }
    }

    // println!("fps = {}", app.fps());
    draw.to_frame(app, &frame).unwrap();
}

// use nannou::prelude::*;
// use svg::Svg;

// fn main() {
//     nannou::app(model).run();
// }

// struct Model {
//     svg: Svg,
// }

// fn model(app: &App) -> Model {
//     app.new_window().size(1200, 800).view(view).build().unwrap();

//     let assets = app.assets_path().unwrap();
//     //    let svg_path = std::path::Path::new("D:\\Processing\\Generative Design\\02_M\\M_1_5_01\\data\\arrow.svg");
//     //let svg_path = assets.join("svg").join("demo2.svg");
//     let svg_path = assets.join("svg").join("tiger.svg");
//     let svg = Svg::load(svg_path.to_path_buf()).expect("failed to load svg");

//     Model { svg }
// }

// fn view(app: &App, m: &Model, frame: Frame) {
//     frame.clear(BLACK);
//     let draw = app.draw();

//     m.svg.paths.iter().for_each(|p| {
//         draw.path()
//             .stroke()
//             //.x_y(app.mouse.x, app.mouse.y)
//             .stroke_weight(1.0)
//             .color(BLACK)
//             .events(p.events.iter().cloned());

//         if let Some(color) = p.fill_color {
//             draw.path()
//                 .fill()
//                 //.x_y(app.mouse.x, app.mouse.y)
//                 .color(color)
//                 .events(p.events.iter().cloned());
//         }

//         if let Some(ref stroke) = p.stroke_style {
//             draw.path()
//                 .stroke()
//                 //.x_y(app.mouse.x, app.mouse.y)
//                 .stroke_weight(stroke.weight)
//                 .color(stroke.color)
//                 .join(stroke.line_join)
//                 .caps(stroke.line_cap)
//                 .events(p.events.iter().cloned());
//         }
//     });

//     // Write the result of our drawing to the window's frame.
//     draw.to_frame(app, &frame).unwrap();
// }
