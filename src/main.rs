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
        // let c = circle();
        // c.shift(DOWN * 100.0);
        // c.move_to(UP * 100.0);

        // let c2 = circle();
        // c2.shift(DOWN * 50.0);
        // self.add(c2.clone());

        // let c3 = circle();
        // c3.shift(LEFT * 50.0);
        // self.add(c3.clone());

        // self.add(c.clone());

        // self.play_many(vec![c.shift(RIGHT * 100.0), c2.to_edge(LEFT)])
        //     .rate_func(BOUNCE_OUT);

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
        }
        self.wait(1.0);
        self.play_many(create);
        self.play_many(edge).rate_func(BOUNCE_OUT);

        // let c2 = self.new(circle());
        // self.act(c2.move_to(point_at(100.0, 100.0)));
        // self.play(c2.to_edge(DOWN)).rate_func(BOUNCE_OUT);

        // self.wait(1.0);

        // let rect = self.new(rectangle());
        // self.show(rect);
        // self.play(rect.move_to(point_at(-100.0, -100.0)));

        // self.play(c2.to_edge(UP)).rate_func(BOUNCE_OUT);

        // let c3 = self.new(circle());
        let c3 = self.circle();
        let r3 = self.rectangle();
        self.act(c3.move_to(point_at(200.0, -100.0)));
        self.act(r3.move_to(point_at(200.0, 100.0)));
        // self.show(r3);
        // self.show(c3);
        self.play(c3.show_creation());
        self.play(r3.show_creation());
        self.play(c3.set_radius(100.0)).rate_func(BOUNCE_OUT);
        self.play(r3.set_height(100.0)).rate_func(QUAD);
        self.play(r3.rotate_by(360.0 * 3.0)).rate_func(QUINT);
        // self.play(r3.scale(2.0)).rate_func(QUINT);

        let t = self.text("Hello!");
        // self.show(t);
        self.play(t.show_creation());
        // let c2 = circle();
        // self.add(c2.clone());
        // self.play(c2.show_creation()).run_time(3.0);

        // self.play(c.shift(RIGHT * 100.0))
        //     .run_time(1.0)
        //     .rate_func(BOUNCE_OUT);

        // self.play(c.shift(RIGHT * 100.0))
        //     .run_time(1.0)
        //     .rate_func(ELASTIC_OUT);

        // self.play(c.move_to(ORIGIN)).run_time(1.0).rate_func(QUAD);

        // self.play(c.to_edge(RIGHT)).rate_func(QUAD);
        // self.play(c.to_edge(LEFT)).rate_func(BOUNCE_OUT);
        // self.play(c.to_edge(UP)).rate_func(ELASTIC_OUT);
        // self.play(c.to_edge(DOWN)).rate_func(QUINT);

        // let cut_times = self.commands.time_stamps();
        // dbg!(&self.commands.run_times());
        // dbg!(&cut_times);
    }
}
