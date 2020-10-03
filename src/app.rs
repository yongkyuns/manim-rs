use nannou::prelude::*;

use crate::scene::{self, Construct, Scene};

pub fn run() {
    nannou::app(scene).update(update).run();
}
fn scene(app: &App) -> Scene {
    let _window = app.new_window().size(640, 480).view(view).build().unwrap();
    // let win_rect = app.main_window().rect().pad_left(20.0);

    let mut scene = scene::scene();
    scene.construct();
    scene
}

fn update(app: &App, scene: &mut Scene, _update: Update) {
    scene.update(app.time);
}

fn view(app: &App, scene: &Scene, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    scene.draw(draw.clone());
    draw.to_frame(app, &frame).unwrap();
}
