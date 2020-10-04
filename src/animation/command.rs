use super::Animation;
use crate::object::RefObject;

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
