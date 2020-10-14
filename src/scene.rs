use crate::animation::{AnimBuilder, Command, Commands, TargetAction, TimedCommand};
use crate::draw::Draw;
use crate::object::RefObject;

use nannou;
use nannou::geom::Rect;

pub trait CommandBuilder {
    fn play(&self, target_action: TargetAction) -> AnimBuilder;
    fn wait(&self, time: f32);
    fn add(&self, object: RefObject);
}

/// Collection of resources used by animation
pub struct Resource {
    window: Rect,
}
impl Resource {
    pub fn new(window: Rect) -> Self {
        Self { window }
    }
    pub fn get_edge_upper(&self) -> f32 {
        self.window.y.end
    }
    pub fn get_edge_lower(&self) -> f32 {
        self.window.y.start
    }
    pub fn get_edge_left(&self) -> f32 {
        self.window.x.start
    }
    pub fn get_edge_right(&self) -> f32 {
        self.window.x.end
    }
}

// pub struct Schedule {
//     pub start_times: Vec<f32>,
// }
// impl Schedule {
//     fn new() -> Self {
//         Self {
//             start_times: Vec::new(),
//         }
//     }
// }
/// Top-level struct that contains animation sequence and object information
pub struct Scene {
    pub commands: Vec<TimedCommand>,
    // pub objects: Vec<Box<dyn Draw>>,
    // pub objects: Vec<&'a dyn Draw>,
    // pub objects: Vec<Object>,
    objects: Vec<RefObject>,
    prev_command: usize,
    resource: Resource,
    // schedule: Schedule,
}
impl Scene {
    pub fn new(window: Rect) -> Self {
        let mut scene = Scene {
            commands: Vec::new(),
            objects: Vec::new(),
            prev_command: 0,
            resource: Resource::new(window),
            // schedule: Schedule::new(),
        };
        scene.wait(0.0); // Put dummy command at the beginning
        scene
    }

    pub fn play(&mut self, mut target_action: TargetAction) -> AnimBuilder {
        target_action.finish_on_drop = false;
        AnimBuilder::new(self, vec![target_action])
    }

    pub fn play_many(&mut self, mut target_actions: Vec<TargetAction>) -> AnimBuilder {
        target_actions
            .iter_mut()
            .for_each(|ta| ta.finish_on_drop = false);
        AnimBuilder::new(self, target_actions)
    }

    pub fn wait(&mut self, time: f32) {
        self.commands.wait(time);
    }
    // pub fn add(&mut self, object: &'a dyn Draw) {
    // pub fn add(&mut self, object: Object) {
    pub fn add(&mut self, object: RefObject) {
        // self.commands.push(Command::Add(object));
        self.commands.add(object);
    }

    pub fn update(&mut self, time: f32) {
        let objects = &mut self.objects;
        self.prev_command = self
            .commands
            .process(self.prev_command, time, objects, &self.resource);

        // // Get current index of animation based on time
        // let from = self.prev_command;
        // let (t_init, upto_static, upto_all) = self.commands.find_index(time);
        // let stride_static = upto_static - from;
        // let stride_all = upto_all - from;
        // let dt = time - t_init;

        // let objects = &mut self.objects;

        // // If stride > 0, non-animation operations (e.g. add/remove)
        // // inbetween need to be processed.
        // if stride_static > 0 {
        //     // If play not reached final state, finish it
        //     self.commands[from].finish_if_needed();
        //     // Draw any non-animation commands
        //     let objects = &mut self.objects;
        //     self.commands
        //         .iter_mut()
        //         .skip(from + 1)
        //         .take(stride)
        //         .for_each(|command| match command {
        //             Command::Add(object) => {
        //                 objects.push(object.clone());
        //             }
        //             _ => (),
        //         });
        // }
        // // Update animation state
        // let command = &mut self.commands[to];
        // if let Command::Play(anim) = command {
        //     anim.update(dt, &self.resource);
        // }

        // self.prev_command = to;
    }

    pub fn draw(&self, nannou_draw: nannou::Draw) {
        // Draw objects in scene
        self.objects
            .iter()
            .for_each(|obj| obj.draw(nannou_draw.clone()));
    }
}

/// Trait to provide user-facing function for making animations
pub trait Construct {
    fn construct(&mut self);
}

pub fn scene(window: Rect) -> Scene {
    Scene::new(window)
}
