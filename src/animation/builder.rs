// #![allow(dead_code)]
use crate::ease::EaseType;
use crate::object::RefObject;
use crate::scene::Scene;

use super::Action;
use super::Animation;
use super::Command;
use super::Status;

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
