#![allow(dead_code)]

mod animation;
mod app;
mod appearance;
mod arena;
mod consts;
mod draw;
mod ease;
mod geom;
mod mobject;
mod object;
mod path;
mod scene;
mod walk;

use animation::{Actionable, TargetAction, UserCommand};
use consts::*;
use geom::point_at;
// use nannou::draw::primitive::rect;
use arena::AddObject;
use scene::{Construct, Scene};

// use nannou::geom::Point2;
use nannou::rand::random_range;

fn main() {
    app::run();
}

impl Construct for Scene {
    fn construct(&mut self) {
        let mut edge: Vec<TargetAction> = Vec::new();
        let mut create: Vec<TargetAction> = Vec::new();
        for _ in 0..600 {
            let x = random_range(-320.0, 320.0);
            let y = random_range(-240.0, 240.0);

            let c = self.circle();
            self.act(c.move_to(point_at(x, y)));
            self.act(c.set_radius(random_range(0.1, 20.0)));
            create.push(c.show_creation());
            edge.push(c.to_edge(DOWN));
            edge.push(c.scale_by(0.3));
            // edge.push(c.set_width(0.3));
        }

        self.play_many(create);
        self.play_many(edge).rate_func(BOUNCE_OUT);

        // let c3 = self.circle();
        // let r3 = self.rectangle();
        // self.act(c3.move_to(point_at(200.0, -100.0)));
        // self.act(r3.move_to(point_at(200.0, 100.0)));
        // self.play(c3.show_creation());
        // self.play(r3.show_creation());

        // self.play(r3.set_height(100.0)).rate_func(QUAD);
        // self.play(c3.set_radius(50.0)).rate_func(BOUNCE_OUT);
        // self.play(r3.rotate_by(360.0 * 3.0)).rate_func(QUINT);

        let t = self.text("Hello!");
        // let t = self.rectangle();

        self.act(t.move_to(point_at(-100.0, 100.0)));
        self.play(t.show_creation()).run_time(1.0);
        // self.play(t.fade_in()).run_time(1.0).rate_func(QUINT);
        self.play(t.scale_by(2.0)).rate_func(QUAD);
        self.play(t.rotate_by(360.0 * 3.0)).rate_func(QUINT);
        // self.play(t.set_width(100.0)).rate_func(QUAD);

        // let cut_times = self.commands.time_stamps();
        // dbg!(&self.commands.run_times());
        // dbg!(&cut_times);
    }
}
