#![allow(dead_code)]
// use crate::draw::Draw;
use crate::ease::EaseType;
use crate::object::RefObject;
use crate::scene::Scene;

use nannou::geom::{Point2, Vector2};

fn interp1(from: f32, to: f32, p: f32) -> f32 {
    // from + (to - from) * p
    from * (1.0 - p) + to * (p)
}
pub trait Interpolate {
    // Update current states based on from, to, and normalized progress.
    fn interp_mut(&mut self, from: &Self, to: &Self, progress: f32)
    where
        Self: Sized;
    fn interp(&self, from: &Self, to: &Self, progress: f32) -> Self
    where
        Self: Sized;
}

impl Interpolate for Point2 {
    fn interp_mut(&mut self, from: &Self, to: &Self, progress: f32) {
        self.x = interp1(from.x, to.x, progress);
        self.y = interp1(from.y, to.y, progress);
    }
    fn interp(&self, from: &Self, to: &Self, progress: f32) -> Self {
        Self {
            x: interp1(from.x, to.x, progress),
            y: interp1(from.y, to.y, progress),
        }
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
    pub fn finish(&self) {
        let object = self.target.borrow_mut();
        // object.act(self.action);
    }
}

impl Drop for TargetAction {
    fn drop(&mut self) {
        if self.finish_on_drop {
            self.finish();
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Play(Animation),
    Wait(f32),
    Add(RefObject),
    Remove(RefObject),
}
impl Command {
    pub fn run_time(&self) -> f32 {
        match self {
            Command::Play(anim) => anim.run_time,
            Command::Wait(t) => *t,
            _ => 0.0,
        }
    }
    pub fn need_finish(&self) -> bool {
        if let Command::Play(anim) = self {
            !anim.is_complete()
        } else {
            false
        }
    }
    pub fn finish_if_needed(&self) {
        if let Command::Play(anim) = self {
            if !anim.is_complete() {}
        }
    }
}

pub trait Commands {
    fn run_times(&self) -> Vec<f32>;
    fn time_stamps(&self) -> Vec<f32>;
    fn find_index(&self, time: f32) -> (f32, usize);
}

impl Commands for Vec<Command> {
    fn run_times(&self) -> Vec<f32> {
        self.iter().map(|c| c.run_time()).collect()
    }
    fn time_stamps(&self) -> Vec<f32> {
        let run_times = self.run_times();
        run_times
            .iter()
            .scan(0.0, |sum, &t| {
                *sum = *sum + t;
                Some(*sum)
            })
            .collect()
    }
    fn find_index(&self, time: f32) -> (f32, usize) {
        let time_stamps = self.time_stamps();
        let idx = time_stamps.iter().rposition(|&t| t <= time).unwrap();
        (time_stamps[idx], idx)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Shift { from: Point2, by: Vector2 },
    MoveTo { from: Point2, to: Point2 },
    Scale { from: f32, to: f32 },
    Rotate { from: f32, to: f32 },
    ShowCreation,
    Write,
    FadeOut,
    Transform,
}
impl Action {
    fn init(&mut self, object: &RefObject) {
        match self {
            Action::Shift { from: _, by } => {
                let pos = object.position();
                *self = Action::Shift { from: pos, by: *by };
            }
            Action::MoveTo { from: _, to } => {
                let pos = object.position();
                *self = Action::MoveTo { from: pos, to: *to };
            }
            // Action::Scale { from: _, to } => {
            //     *self = Action::Scale { from: , to: *to };
            // }
            _ => (),
        }
    }
    fn update(&self, object: &mut RefObject, progress: f32) {
        match self {
            Action::Shift { from, by } => {
                let ref to = *from + *by;
                let now = object.position().interp(from, to, progress);
                object.set_position(now);
            }
            _ => (),
        }
    }
}

pub trait SetPosition {
    fn position(&self) -> Point2;
    fn set_position(&mut self, to: Point2);
}

pub trait Animate: SetPosition {
    fn shift(&self, by: Vector2) -> TargetAction;
    fn act(&mut self, action: Action);
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
    fn init(&mut self) {
        self.action.init(&self.object);
    }
    pub fn is_complete(&self) -> bool {
        if let Status::Complete = self.status {
            true
        } else {
            false
        }
    }
    fn update_status(&mut self, t: f32) {
        if t > 0.0 {
            if self.status == Status::NotStarted {
                self.action.init(&self.object);
            }
            self.status = Status::Animating(t / self.run_time);
        }
    }
    pub fn update(&mut self, t: f32) {
        let t = t.min(self.run_time);

        self.update_status(t);
        let p = self.rate_func.calculate(t / self.run_time);
        let object = &mut self.object;

        self.action.update(object, p);
    }
}

#[derive(Debug)]
pub struct AnimBuilder<'a> {
    scene: &'a mut Scene,
    object: RefObject,
    action: Action,
    run_time: f32,
    rate_func: EaseType,
}
impl<'a> AnimBuilder<'a> {
    pub fn new(scene: &'a mut Scene, object: RefObject, action: Action) -> Self {
        AnimBuilder {
            scene,
            object,
            action,
            run_time: 1.0,
            rate_func: EaseType::Linear,
        }
    }
    pub fn run_time(mut self, duration: f32) -> Self {
        self.run_time = duration;
        self
    }
    pub fn rate_func(mut self, rate_func: EaseType) -> Self {
        self.rate_func = rate_func;
        self
    }
}

impl<'a> Drop for AnimBuilder<'a> {
    fn drop(&mut self) {
        let anim = Animation {
            object: self.object.clone(),
            action: self.action.clone(),
            run_time: self.run_time,
            rate_func: self.rate_func,
            status: Status::NotStarted,
        };
        self.scene.commands.push(Command::Play(anim));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consts::*;
    use crate::object::circle::circle;
    use crate::scene::scene;
    #[test]
    fn simple_shift() {
        let c = circle();
        let mut scene = scene();
        scene.add(c.clone());
        scene.wait(1.0);
        scene
            .play(c.shift(RIGHT * 100.0))
            .run_time(1.0)
            .rate_func(BOUNCE);
    }
}
