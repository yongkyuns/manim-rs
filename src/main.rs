#![allow(dead_code)]

mod animation;
mod app;
mod consts;
mod draw;
mod ease;
mod mobject;
mod object;
mod path;
mod scene;
mod walk;

use animation::{Animate, CommandBuilder, TargetAction};
use consts::*;
use object::circle::circle;
use scene::{Construct, Scene};

use nannou::geom::Point2;
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

        let mut anims: Vec<TargetAction> = Vec::new();
        for _ in 0..6000 {
            let x = random_range(-320.0, 320.0);
            let y = random_range(-240.0, 240.0);

            let c = circle();
            c.move_to(Point2 { x, y });
            self.add(c.clone());
            // anims.push(c.move_to(ORIGIN));
            anims.push(c.to_edge(LEFT));
        }
        self.play_many(anims).run_time(5.0).rate_func(BOUNCE_OUT);

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
