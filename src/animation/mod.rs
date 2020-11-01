// #![allow(dead_code)]
pub use self::action::{Action, Direction};
pub use self::builder::AnimBuilder;
pub use self::command::{Command, CommandBuilder, Commands, TimedCommand};

use crate::ease::EaseType;
use crate::object::RefObject;
use crate::scene::Resource;

pub mod action;
pub mod builder;
pub mod command;

// use nannou::geom::{Point2, Vector2};
use nannou::lyon::math::{point, Point, Vector};

fn interp1(from: f32, to: f32, p: f32) -> f32 {
    from * (1.0 - p) + to * (p)
}
pub trait Interpolate {
    // Update current states based on from, to, and normalized progress.
    fn interp_mut(&mut self, other: &Self, progress: f32)
    where
        Self: Sized;
    fn interp(&self, other: &Self, progress: f32) -> Self
    where
        Self: Sized;
}

impl Interpolate for Point {
    fn interp_mut(&mut self, other: &Self, progress: f32) {
        self.x = interp1(self.x, other.x, progress);
        self.y = interp1(self.y, other.y, progress);
    }
    fn interp(&self, other: &Self, progress: f32) -> Self {
        let x = interp1(self.x, other.x, progress);
        let y = interp1(self.y, other.y, progress);
        point(x, y)
    }
}

pub struct TargetAction {
    pub target: RefObject,
    pub action: Action,
    pub finish_on_drop: bool,
}
impl TargetAction {
    pub fn new(target: RefObject, action: Action, finish_on_drop: bool) -> Self {
        Self {
            target,
            action,
            finish_on_drop,
        }
    }
    pub fn finish(&mut self) {
        self.action.complete(&mut self.target);
    }
}

impl Drop for TargetAction {
    fn drop(&mut self) {
        if self.finish_on_drop {
            self.finish();
        }
    }
}

pub trait SetPosition {
    fn position(&self) -> Point;
    fn set_position(&mut self, to: Point);
}

pub trait Animate: SetPosition {
    fn shift(&self, by: Vector) -> TargetAction;
    fn move_to(&self, to: Point) -> TargetAction;
    fn to_edge(&self, edge: Vector) -> TargetAction;
}

#[derive(Debug, PartialEq)]
pub enum Status {
    NotStarted,
    Animating(f32),
    Complete,
}

#[derive(Debug, PartialEq)]
pub struct Animation {
    // pub property: &'a mut dyn Interpolate,
    object: RefObject,
    action: Action,
    run_time: f32,
    rate_func: EaseType,
    status: Status,
}

impl Animation {
    pub fn new(object: RefObject, action: Action) -> Self {
        // pub fn new(property: Rc<RefCell<dyn Interpolate>>, action: Action) -> Self {
        let rate_func = EaseType::Linear;
        let run_time = 1.0;
        let status = Status::NotStarted;
        Animation {
            object,
            action,
            rate_func,
            run_time,
            status,
        }
    }
    // Set object to final state in animation
    pub fn finish(&mut self) {
        self.status = Status::Complete;
    }
    // Initialize animation state with current object state
    fn init(&mut self, resource: &Resource) {
        self.action.init(&self.object, resource);
    }
    // Determine whether animation is complete
    pub fn is_complete(&self) -> bool {
        if let Status::Complete = self.status {
            true
        } else {
            false
        }
    }
    // Update animation status and time
    fn update_status(&mut self, t: f32, resource: &Resource) {
        if t > 0.0 {
            if self.status == Status::NotStarted {
                self.action.init(&self.object, resource);
            }
            self.status = Status::Animating(t / self.run_time);
        }
    }
    // Main update function for progressing through animation
    pub fn update(&mut self, t: f32, resource: &Resource) {
        let t = t.min(self.run_time);

        self.update_status(t, resource);
        let p = self.rate_func.calculate(t / self.run_time);
        let object = &mut self.object;

        self.action.update(object, p);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consts::*;
    use crate::object::circle::circle;
    use crate::scene::scene;
    use nannou::geom::{Range, Rect};
    #[test]
    fn simple_shift() {
        let c = circle();
        let win = Rect {
            x: Range {
                start: 0.0,
                end: 300.0,
            },
            y: Range {
                start: 0.0,
                end: 300.0,
            },
        };

        let mut scene = scene(win);
        scene.add(c.clone());
        scene.wait(1.0);
        scene
            .play(c.shift(RIGHT * 100.0))
            .run_time(1.0)
            .rate_func(BOUNCE);
    }
}
