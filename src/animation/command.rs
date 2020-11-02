use super::{AnimBuilder, Animation, TargetAction};
use crate::object::RefObject;
use crate::scene::Resource;

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
            if !anim.is_complete() {} //TODO fix completion later
        }
    }
}
pub trait CommandBuilder {
    fn play(&mut self, target_action: TargetAction) -> AnimBuilder;
    fn wait(&mut self, time: f32);
    fn add(&mut self, object: RefObject);
}

#[derive(Debug, PartialEq)]
pub struct TimedCommand {
    event_time: f32,
    inner: Command,
}
impl TimedCommand {
    pub fn finish_if_needed(&self) {
        self.inner.finish_if_needed();
    }
}

pub trait Commands {
    fn run_times(&self) -> Vec<f32>;
    fn time_stamps(&self) -> Vec<f32>;
    fn find_index(&self, time: f32) -> (f32, usize, usize);
    fn end_time(&self) -> f32;
    fn wait(&mut self, t: f32);
    fn add(&mut self, object: RefObject);
    fn play(&mut self, animations: Vec<Animation>);
    fn process(
        &mut self,
        idx_start: usize,
        time: f32,
        objects: &mut Vec<RefObject>,
        resource: &Resource,
    ) -> usize;
}

impl Commands for Vec<TimedCommand> {
    fn run_times(&self) -> Vec<f32> {
        self.iter().map(|c| c.inner.run_time()).collect()
    }

    fn time_stamps(&self) -> Vec<f32> {
        // let mut stamps: Vec<f32> = self
        //     .run_times()
        //     .iter()
        //     .scan(0.0, |sum, &t| {
        //         *sum = *sum + t;
        //         Some(*sum)
        //     })
        //     .collect();
        // // time-stamps need to be shifted one index to the right
        // // to mark the beginning of each animation, not end
        // stamps.insert(0, 0.0);
        // stamps.pop();
        // stamps

        self.iter().map(|c| c.event_time).collect()
    }

    fn find_index(&self, time: f32) -> (f32, usize, usize) {
        let idx_end = self.iter().rposition(|c| c.event_time <= time).unwrap();
        let start_time = self[idx_end].event_time;
        let idx_start = self
            .iter()
            .position(|c| c.event_time == start_time && c.inner.run_time() > 0.0)
            .unwrap_or(idx_end);
        (start_time, idx_start, idx_end)
    }

    fn end_time(&self) -> f32 {
        if let Some(cmd) = self.last() {
            cmd.event_time + cmd.inner.run_time()
        } else {
            0.0
        }
    }

    fn wait(&mut self, t: f32) {
        self.push(TimedCommand {
            event_time: self.end_time(),
            inner: Command::Wait(t),
        });
    }

    fn add(&mut self, object: RefObject) {
        self.push(TimedCommand {
            event_time: self.end_time(),
            inner: Command::Add(object),
        });
    }

    fn play(&mut self, animations: Vec<Animation>) {
        let start_time = self.end_time();
        for anim in animations.into_iter() {
            self.push(TimedCommand {
                event_time: start_time,
                inner: Command::Play(anim),
            });
        }
    }

    fn process(
        &mut self,
        idx_prev: usize,
        time: f32,
        objects: &mut Vec<RefObject>,
        resource: &Resource,
    ) -> usize {
        let (t_begin, idx_start, idx_end) = self.find_index(time);
        let dt = time - t_begin;

        // Finish animation, or add/remove objects
        self.iter_mut()
            .skip(idx_prev)
            .take(idx_start - idx_prev)
            .for_each(|c| {
                let cmd = &mut c.inner;
                match cmd {
                    Command::Play(anim) => anim.finish(),
                    Command::Add(object) => objects.push(object.clone()),
                    _ => (),
                }
            });

        // Update animation
        self.iter_mut()
            .skip(idx_start)
            .take(idx_end - idx_start + 1)
            .for_each(|c| {
                let cmd = &mut c.inner;
                match cmd {
                    Command::Play(anim) => anim.update(dt, resource),
                    _ => (),
                }
            });

        idx_start
    }
}
