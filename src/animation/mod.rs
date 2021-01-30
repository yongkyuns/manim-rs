// #![allow(dead_code)]
pub use self::action::{Action, Direction};
pub use self::builder::AnimBuilder;
pub use self::command::{Command, RunCommand, TimedCommand, UserCommand};

use crate::arena::{Id, Object};
use crate::ease::EaseType;
use crate::geom::{GetPosition, Point, Vector};
use crate::scene::Resource;

pub mod action;
pub mod builder;
pub mod command;

pub fn lerp(from: f32, to: f32, p: f32) -> f32 {
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

#[derive(Debug, PartialEq)]
pub struct TargetAction {
    pub target: Id,
    pub action: Action,
    pub finish_on_drop: bool,
}

impl TargetAction {
    pub fn new(target: Id, action: Action, finish_on_drop: bool) -> Self {
        Self {
            target,
            action,
            finish_on_drop,
        }
    }
    pub fn finish(&mut self, object: &mut Object) {
        self.action.complete(object);
    }
}

impl Drop for TargetAction {
    fn drop(&mut self) {
        if self.finish_on_drop {
            // self.finish();
        }
    }
}

pub trait PathCompletion {
    fn completion(&self) -> f32;
    fn set_completion(&mut self, completion: f32);
}

/// Describes the action and target object.
/// Returns `TargetAction` which can change object instantly, or
/// Furuther gets converted to `Animation` which contains duration and interpolation function
pub trait Animate {
    fn shift(&self, by: Vector) -> TargetAction;
    fn move_to(&self, to: Point) -> TargetAction;
    fn to_edge(&self, edge: Vector) -> TargetAction;
    fn show_creation(&self) -> TargetAction;
}

#[derive(Debug, PartialEq)]
pub enum Status {
    NotStarted,
    Animating(f32),
    Complete,
}

#[derive(Debug, PartialEq)]
pub struct Animation {
    object: Id,
    action: Action,
    run_time: f32,
    rate_func: EaseType,
    status: Status,
}

impl Animation {
    pub fn new(object: Id, action: Action) -> Self {
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
    pub fn finish(&mut self, object: &mut Object) {
        if !(self.status == Status::Complete) {
            // let object = &mut self.object;
            self.action.update(object, 1.0);
            self.status = Status::Complete;
        }
    }
    // Initialize animation state with current object state
    fn init(&mut self, object: &mut Object, resource: &Resource) {
        self.action.init(object, resource);
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
    fn update_status(&mut self, object: &mut Object, t: f32, resource: &Resource) {
        if t > 0.0 {
            if self.status == Status::NotStarted {
                self.action.init(object, resource);
            }
            self.status = Status::Animating(t / self.run_time);
        }
    }
    // Main update function for progressing through animation
    pub fn update(&mut self, object: &mut Object, t: f32, resource: &Resource) {
        let t = t.min(self.run_time);

        self.update_status(object, t, resource);
        let p = self.rate_func.calculate(t / self.run_time);
        // let object = &mut self.object;

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
        scene.add(c);
        scene.wait(1.0);
        scene
            .play(c.shift(RIGHT * 100.0))
            .run_time(1.0)
            .rate_func(BOUNCE);
    }
}
