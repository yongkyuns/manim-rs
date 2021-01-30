use super::{AnimBuilder, Animation, TargetAction};
use crate::appearance::Visibility;
use crate::arena::{Arena, Id, Object};
use crate::scene::Resource;

/// Types of commands available. Vector of `TimedCommand` constructs the
/// sequence of motion generated by the `Scene`.
#[derive(Debug, PartialEq)]
pub enum Command {
    Play(Animation),   // Play motion interpolated through specified time
    Act(TargetAction), // Instantly takes change
    Show(Id),          // Makes object visible
    Remove(Id),        // Removes object from teh scene
    Wait(f32),         // Pauses for the duration
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
pub trait UserCommand {
    fn play(&mut self, target_action: TargetAction) -> AnimBuilder;
    fn act(&mut self, target_action: TargetAction);
    fn wait(&mut self, time: f32);
    fn new(&mut self, object: Object) -> Id;
    fn show(&mut self, id: Id);
    fn remove(&mut self, id: Id);
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

/// Executes commands and provides information
pub trait RunCommand {
    fn run_times(&self) -> Vec<f32>;
    fn time_stamps(&self) -> Vec<f32>;
    fn find_index(&self, time: f32) -> (f32, usize, usize);
    fn end_time(&self) -> f32;
    fn wait(&mut self, t: f32);
    fn show(&mut self, object: Id);
    fn act(&mut self, ta: TargetAction);
    fn play(&mut self, animations: Vec<Animation>);
    fn process(
        &mut self,
        idx_start: usize,
        time: f32,
        objects: &mut Arena<Object>,
        resource: &Resource,
    ) -> usize;
}

impl RunCommand for Vec<TimedCommand> {
    fn run_times(&self) -> Vec<f32> {
        self.iter().map(|c| c.inner.run_time()).collect()
    }

    fn time_stamps(&self) -> Vec<f32> {
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
    fn show(&mut self, object: Id) {
        self.push(TimedCommand {
            event_time: self.end_time(),
            inner: Command::Show(object),
        });
    }
    fn act(&mut self, ta: TargetAction) {
        self.push(TimedCommand {
            event_time: self.end_time(),
            inner: Command::Act(ta),
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
        objects: &mut Arena<Object>,
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
                    Command::Play(anim) => {
                        objects.get_mut(anim.object.0).map(|obj| anim.finish(obj));
                    }
                    Command::Act(ta) => {
                        objects
                            .get_mut(ta.target.0)
                            .map(|obj| ta.action.complete(obj));
                    }
                    Command::Show(id) => {
                        objects.get_mut(id.0).map(|obj| obj.show());
                    }
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
                    Command::Play(anim) => {
                        objects
                            .get_mut(anim.object.0)
                            .map(|obj| anim.update(obj, dt, resource));
                    }
                    _ => (),
                }
            });

        idx_start
    }
}
