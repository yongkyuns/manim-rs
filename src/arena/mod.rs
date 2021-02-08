pub use generational_arena::{Arena, Index};

pub use tree::{Id, NodeArena, NodeIndex, Object};

pub use index::{CircleAction, CircleId, RectangleAction, RectangleId, TextAction, TextId};

pub use property::rotate::Rotate;

mod index;
mod property;
mod tree;

pub trait HasArena {
    fn add(&mut self, object: Object) -> Id;
    fn get_mut(&mut self, index: &Id) -> Option<&mut Object>;
    fn get(&self, index: &Id) -> Option<&Object>;
    fn get_parent(&self, index: &Id) -> Option<&Object>;
}

pub trait AddObject {
    fn circle(&mut self) -> CircleId;
    fn rectangle(&mut self) -> RectangleId;
    fn text(&mut self, text: &str) -> TextId;
}
