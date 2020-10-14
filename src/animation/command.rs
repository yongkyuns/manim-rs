use super::Animation;
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
        let idx_all = self.iter().rposition(|c| c.event_time <= time).unwrap();
        let idx_non_anim = self
            .iter()
            .rposition(|c| c.event_time <= time && c.inner.run_time() == 0.0)
            .unwrap();
        (self[idx_all].event_time, idx_non_anim, idx_all)
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
        idx_start: usize,
        time: f32,
        objects: &mut Vec<RefObject>,
        resource: &Resource,
    ) -> usize {
        let (t_begin, idx_static, idx_all) = self.find_index(time);
        let dt = time - t_begin;
        let stride = idx_all - idx_start;

        if stride > 0 {
            self.iter_mut()
                .skip(idx_start + 1)
                .take(stride)
                .for_each(|c| {
                    let cmd = &mut c.inner;
                    match cmd {
                        Command::Play(anim) => anim.update(dt, resource),
                        Command::Add(object) => objects.push(object.clone()),
                        _ => (),
                    }
                })
        } else {
            let cmd = &mut self[idx_start].inner;
            if let Command::Play(anim) = cmd {
                anim.update(dt, resource);
            }
        }
        idx_static
    }
}
