use nannou::prelude::*;

use crate::scene::{self, Construct, Scene};

pub fn run() {
    nannou::app(scene).update(update).run();
}
fn scene(app: &App) -> Scene {
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
    println!("{}", app.fps());
    let draw = app.draw();
    draw.background().color(BLACK);
    scene.draw(draw.clone());
    draw.to_frame(app, &frame).unwrap();
}
