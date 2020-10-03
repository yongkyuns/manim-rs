// pub mod ease;
pub mod animation;
pub mod primitive;

use animation::{Animator, EaseType};
use nannou::prelude::*;
use primitive::Arrow;

struct Model {
    animator: Animator<Arrow>,
}

fn model(app: &App) -> Model {
    app.new_window().size(512, 512).view(view).build().unwrap();

    let r = app.window_rect();
    let mut animator = Animator::new();

    for r in r.subdivisions_iter() {
        let start = r.xy();
        let end = start + pt2(1.0, 1.0);
        animator.register(Arrow { start, end });
    }

    Model { animator }
}

fn main() {
    nannou::app(model).update(update).run();
}

fn update(app: &App, model: &mut Model, _update: Update) {
    if app.mouse.buttons.left().is_down() {
        for tween in model.animator.tweens.iter_mut() {
            let start = tween.initial.start;
            let end = app.mouse.position();
            tween.set_target(Arrow { start, end });
        }

        model.animator.start(EaseType::Linear, app.time, 0.2);
    }
    model.animator.update(app.time);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    for tween in &model.animator.tweens {
        let arrow = &tween.current;
        if arrow.start != arrow.end {
            draw.arrow().weight(3.0).points(arrow.start, arrow.end);
        }
    }

    let line = draw.line().weight(5.0);
    draw.arrow()
        .color(YELLOW)
        .weight(3.0)
        .points(pt2(0.0, -100.0), pt2(-100.0, -100.0));
    line.color(RED).points(pt2(0.0, 0.0), pt2(0.0, 200.0));

    println!("fps = {}", app.fps());
    draw.to_frame(app, &frame).unwrap();
}
