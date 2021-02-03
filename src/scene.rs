use crate::animation::{AnimBuilder, RunCommand, TargetAction, TimedCommand, UserCommand};
use crate::arena::{AddObject, Arena, CircleId, HasArena, Id, Index};
use crate::arena::{NodeArena, NodeIndex, Object, RectangleId};
use crate::draw::Draw;
use crate::object::circle::circle;
use crate::object::rectangle::rectangle;

// use std::slice::IterMut;

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
    objects: Arena<Object>,
    prev_command: usize,
    resource: Resource,
}

impl Scene {
    pub fn new(window: Rect) -> Self {
        let mut scene = Scene {
            commands: Vec::new(),
            objects: Arena::new(),
            prev_command: 0,
            resource: Resource::new(window),
        };
        scene.wait(0.0); // Put dummy command at the beginning
        scene
    }

    pub fn play_many(&mut self, target_actions: Vec<TargetAction>) -> AnimBuilder {
        AnimBuilder::new(self, target_actions)
    }

    pub fn update(&mut self, time: f32) {
        // dbg!(&self.prev_command);
        self.prev_command =
            self.commands
                .process(self.prev_command, time, &mut self.objects, &self.resource);
    }

    pub fn draw(&self, nannou_draw: nannou::Draw) {
        for (_idx, object) in &self.objects {
            object.draw(nannou_draw.clone());
        }
    }
}

impl UserCommand for Scene {
    fn play(&mut self, target_action: TargetAction) -> AnimBuilder {
        AnimBuilder::new(self, vec![target_action])
    }
    fn act(&mut self, target_action: TargetAction) {
        self.commands.act(target_action);
    }
    fn wait(&mut self, time: f32) {
        self.commands.wait(time);
    }
    fn show<T>(&mut self, object: T)
    where
        T: Into<Index> + Sized + Copy,
    {
        let id: Index = object.into();
        self.commands.show(NodeIndex(id));
    }
    fn remove<T>(&mut self, object: T)
    where
        T: Into<Index> + Sized + Copy,
    {
        let id: Index = object.into();
        self.objects.delete(NodeIndex(id));
    }
    // fn new(&mut self, object: Object) -> Id {
    //     self.objects.add(object) // Add object to graph
    //                              // self.commands.add(idx); // add new command
    // }
}

impl HasArena for Scene {
    fn add(&mut self, object: Object) -> Id {
        Id::new(self.objects.insert(object))
    }
    fn get_mut(&mut self, index: &Id) -> Option<&mut Object> {
        self.objects.get_mut(index.0)
    }
    fn get(&self, index: &Id) -> Option<&Object> {
        self.objects.get(index.0)
    }
    fn get_parent(&self, index: &Id) -> Option<&Object> {
        self.objects
            .get(index.0)
            .and_then(|object| object.parent.and_then(|parent| self.objects.get(parent)))
    }
}

impl AddObject for Scene {
    fn circle(&mut self) -> CircleId {
        let index = self.objects.add(circle());
        CircleId(index.0)
    }
    fn rectangle(&mut self) -> RectangleId {
        let index = self.objects.add(rectangle());
        RectangleId(index.0)
    }
}

/// Trait to provide user-facing function for making animations
pub trait Construct {
    fn construct(&mut self);
}

pub fn scene(window: Rect) -> Scene {
    Scene::new(window)
}
