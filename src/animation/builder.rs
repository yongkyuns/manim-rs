// #![allow(dead_code)]
use crate::consts::DEFAULT_RUNTIME;
use crate::ease::EaseType;
use crate::scene::Scene;

use super::Animation;
use super::RunCommand;
use super::Status;
use super::{Action, TargetAction};

pub struct AnimBuilder<'a> {
    scene: &'a mut Scene,
    target_actions: Vec<TargetAction>,
    run_time: f32,
    rate_func: EaseType,
}

impl<'a> AnimBuilder<'a> {
    pub fn new(scene: &'a mut Scene, target_actions: Vec<TargetAction>) -> Self {
        let mut rate_func = EaseType::Linear;
        for ta in target_actions.iter() {
            if ta.action == Action::ShowCreation {
                rate_func = EaseType::Quad;
                break;
            }
        }
        AnimBuilder {
            scene,
            target_actions,
            run_time: DEFAULT_RUNTIME,
            rate_func,
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
        let run_time = self.run_time;
        let rate_func = self.rate_func;
        // let start_time = *self.scene.commands.time_stamps().last().unwrap();
        let scene = &mut self.scene;

        let mut animations: Vec<Animation> = Vec::new();
        for ta in self.target_actions.iter() {
            let anim = Animation {
                object: ta.target.clone(),
                action: ta.action.clone(),
                run_time,
                rate_func,
                status: Status::NotStarted,
            };
            animations.push(anim);
        }
        // let animations = self
        //     .target_actions
        //     .iter()
        //     .map(|ta| {
        //         let object = ta.target.clone();
        //         let action = ta.action.clone();
        //         Animation {
        //             object,
        //             action,
        //             run_time,
        //             rate_func,
        //             status,
        //         }
        //     })
        //     .collect();
        scene.commands.play(animations);
    }
}
