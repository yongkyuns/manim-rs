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

use animation::{Animate, Commands};
use consts::*;
use object::circle::circle;
use scene::{Construct, Scene};

fn main() {
    app::run();
}

impl Construct for Scene {
    fn construct(&mut self) {
        let c = circle();
        c.shift(DOWN * 100.0);
        self.add(c.clone());
        self.wait(1.0);
        self.play(c.shift(UR * 100.0))
            .run_time(1.0)
            .rate_func(BOUNCE_OUT);

        self.play(c.shift(LEFT * 100.0))
            .run_time(1.0)
            .rate_func(ELASTIC_OUT);

        let cut_times = self.commands.time_stamps();
        dbg!(&cut_times);
    }
}
