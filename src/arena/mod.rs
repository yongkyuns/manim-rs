pub use generational_arena::Arena;

pub use tree::{Id, NodeArena, Object};

use crate::geom::Point;

mod tree;

pub trait SetPositionArena<Arena>
where
    Arena: HasArena,
{
    fn move_by(&self, arena: &mut Arena, x: f32, y: f32);
    fn move_to(&self, arena: &mut Arena, x: f32, y: f32);
}

pub trait GetPositionArena<Arena>
where
    Arena: HasArena,
{
    fn position(&self, arena: &Arena) -> Option<Point>;
}

pub trait HasArena {
    fn add(&mut self, object: Object) -> Id;
    fn get_mut(&mut self, index: &Id) -> Option<&mut Object>;
    fn get(&self, index: &Id) -> Option<&Object>;
    fn get_parent(&self, index: &Id) -> Option<&Object>;
}
