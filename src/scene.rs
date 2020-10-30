use crate::animation::{AnimBuilder, CommandBuilder, Commands, TargetAction, TimedCommand};
use crate::draw::Draw;
use crate::object::RefObject;

use nannou;
use nannou::geom::Rect;

/// Collection of resources used by animation
pub struct Resource {
    window: Rect,
}
impl Resource {
    pub fn new(window: Rect) -> Self {
        Self { window }
    }
    pub fn edge_upper(&self) -> f32 {
        self.window.y.end
    }
    pub fn edge_lower(&self) -> f32 {
        self.window.y.start
    }
    pub fn edge_left(&self) -> f32 {
        self.window.x.start
    }
    pub fn edge_right(&self) -> f32 {
        self.window.x.end
    }
}

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

    pub fn play_many(&mut self, mut target_actions: Vec<TargetAction>) -> AnimBuilder {
        target_actions
            .iter_mut()
            .for_each(|ta| ta.finish_on_drop = false);
        AnimBuilder::new(self, target_actions)
    }

    pub fn update(&mut self, time: f32) {
        let objects = &mut self.objects;
        self.prev_command = self
            .commands
            .process(self.prev_command, time, objects, &self.resource);
    }

    pub fn draw(&self, nannou_draw: nannou::Draw) {
        // Draw objects in scene
        self.objects
            .iter()
            .for_each(|obj| obj.draw(nannou_draw.clone()));
    }
}

impl CommandBuilder for Scene {
    fn play(&mut self, target_action: TargetAction) -> AnimBuilder {
        AnimBuilder::new(self, vec![target_action])
    }

    fn wait(&mut self, time: f32) {
        self.commands.wait(time);
    }
    fn add(&mut self, object: RefObject) {
        self.commands.add(object);
    }
}

/// Trait to provide user-facing function for making animations
pub trait Construct {
    fn construct(&mut self);
}

pub fn scene(window: Rect) -> Scene {
    Scene::new(window)
}
