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

    for r in r.subdivisions_iter() {
        let start = r.xy();
        let end = start + pt2(1.0, 1.0);
        tweener.register(Arrow { start, end });
    }

    Model { tweener }
}

fn main() {
    nannou::app(model).update(update).run();
}

fn update(app: &App, model: &mut Model, _update: Update) {
    if app.mouse.buttons.left().is_down() {
        for tween in model.tweener.tweens.iter_mut() {
            let start = tween.initial.start;
            let end = app.mouse.position();
            tween.set_target(Arrow { start, end });
        }

        model.tweener.start(EaseType::BounceOut, app.time, 1.0);
    }
    model.tweener.update(app.time);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    for tween in &model.tweener.tweens {
        let arrow = &tween.current;
        if arrow.start != arrow.end {
            draw.arrow().weight(3.0).points(arrow.start, arrow.end);
        }
    }

    println!("fps = {}", app.fps());
    draw.to_frame(app, &frame).unwrap();
}
