use crate::animation::{AnimBuilder, Command, Commands, TargetAction};
use crate::draw::Draw;
use crate::object::RefObject;

use nannou;
use nannou::geom::Rect;

pub trait CommandBuilder {
    fn play(&self, target_action: TargetAction) -> AnimBuilder;
    fn wait(&self, time: f32);
    fn add(&self, object: RefObject);
}

pub struct Scene {
    pub commands: Vec<Command>,
    // pub objects: Vec<Box<dyn Draw>>,
    // pub objects: Vec<&'a dyn Draw>,
    // pub objects: Vec<Object>,
    objects: Vec<RefObject>,
    prev_command: usize,
    window: Rect,
}
impl Scene {
    pub fn new(window: Rect) -> Self {
        let mut scene = Scene {
            commands: Vec::new(),
            objects: Vec::new(),
            prev_command: 0,
            window,
        };
        scene.wait(0.0); //Need animation to start from 0.0 sec
        scene
    }
    pub fn play(&mut self, mut target_action: TargetAction) -> AnimBuilder {
        target_action.finish_on_drop = false;
        AnimBuilder::new(
            self,
            target_action.target.clone(),
            target_action.action.clone(),
        )
    }
    pub fn wait(&mut self, time: f32) {
        self.commands.push(Command::Wait(time));
    }
    // pub fn add(&mut self, object: &'a dyn Draw) {
    // pub fn add(&mut self, object: Object) {
    pub fn add(&mut self, object: RefObject) {
        self.commands.push(Command::Add(object));
    }

    pub fn update(&mut self, time: f32) {
        // Get current index of animation based on time
        let from = self.prev_command;
        let (t_init, to) = self.commands.find_index(time);
        let stride = to - from;
        let dt = time - t_init;

        // If stride > 0, non-animation operations (e.g. add/remove)
        // inbetween need to be processed.
        if stride > 0 {
            // If play not reached final state, finish it
            self.commands[from].finish_if_needed();
            // Draw any non-animation commands
            let objects = &mut self.objects;
            self.commands
                .iter_mut()
                .skip(from)
                .take(stride)
                .for_each(|command| match command {
                    // Command::Play(ref mut anim) => anim.update(dt),
                    Command::Add(object) => objects.push(object.clone()),
                    _ => (),
                });
        }
        // Update animation state
        let command = &mut self.commands[to];
        if let Command::Play(anim) = command {
            anim.update(dt);
        }

        self.prev_command = to;
    }
    pub fn draw(&self, nannou_draw: nannou::Draw) {
        // Draw objects in scene
        self.objects
            .iter()
            .for_each(|obj| obj.draw(nannou_draw.clone()));
    }
}

pub trait Construct {
    fn construct(&mut self);
}

// impl CommandBuilder for Scene {}

pub fn scene(window: Rect) -> Scene {
    Scene::new(window)
}

// impl CommandBuilder for RefCell<Scene> {
//     fn play(&self, target_action: (RefObject, Action)) -> AnimBuilder {
//         self.borrow_mut().play(target_action)
//     }
//     fn wait(&self, time: f32) {
//         self.borrow_mut().wait(time);
//     }
//     fn add(&self, object: RefObject) {
//         self.borrow_mut().add(object);
//     }
// }
