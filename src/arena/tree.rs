use super::{Arena, HasArena};
use crate::animation::{Action, Animate, Direction, PathCompletion, TargetAction};
use crate::appearance::Visibility;
use crate::consts::*;
use crate::draw::Draw;
use crate::geom::{point, GetPosition, Point, SetPosition, Vector};
use crate::object::Object as InnerObject;
use crate::scene::Scene;

use generational_arena::Index;

use super::{GetPositionArena, SetPositionArena};

pub type Object = Node; // Treat Node like an object
pub type Id = NodeIndex; // Short-hand for readability

#[derive(Debug, PartialEq)]
pub struct Node {
    pub parent: Option<Index>,
    pub child: Option<Index>,
    pub inner: InnerObject,
}

impl Node {
    pub fn new(object: InnerObject) -> Self {
        Self {
            parent: None,
            child: None,
            inner: object,
        }
    }
    pub fn set_child(&mut self, idx: Index) {
        self.child = Some(idx);
    }
    pub fn set_parent(&mut self, idx: Index) {
        self.parent = Some(idx);
    }
}

impl SetPosition for Node {
    fn position_mut(&mut self) -> &mut Point {
        SetPosition::position_mut(&mut self.inner)
    }
}

impl GetPosition for Node {
    fn position(&self) -> Point {
        GetPosition::position(&self.inner)
    }
}

impl Draw for Node {
    fn draw(&self, draw: nannou::Draw) {
        Draw::draw(&self.inner, draw);
    }
}

impl Visibility for Node {
    fn visible_mut(&mut self) -> &mut bool {
        Visibility::visible_mut(&mut self.inner)
    }
    fn is_visible(&self) -> bool {
        Visibility::is_visible(&self.inner)
    }
}

impl PathCompletion for Node {
    fn completion(&self) -> f32 {
        PathCompletion::completion(&self.inner)
    }
    fn set_completion(&mut self, completion: f32) {
        PathCompletion::set_completion(&mut self.inner, completion);
    }
}

pub trait NodeArena {
    fn add(&mut self, node: Node) -> NodeIndex;
    fn delete(&mut self, ni: NodeIndex);
}

impl NodeArena for Arena<Node> {
    fn add(&mut self, node: Node) -> NodeIndex {
        NodeIndex(self.insert(node))
    }
    fn delete(&mut self, ni: NodeIndex) {
        self.remove(ni.0);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NodeIndex(pub Index);

impl NodeIndex {
    pub fn new(idx: Index) -> Self {
        Self(idx)
    }
    pub fn set_child(&self, scene: &mut Scene, child: &NodeIndex) {
        scene.get_mut(&self).map(|obj| obj.set_child(child.0));
    }
    pub fn set_parent(&self, scene: &mut Scene, parent: &NodeIndex) {
        scene.get_mut(&self).map(|obj| obj.set_parent(parent.0));
    }
}

impl<Arena> SetPositionArena<Arena> for NodeIndex
where
    Arena: HasArena,
{
    fn move_by(&self, arena: &mut Arena, x: f32, y: f32) {
        arena.get_mut(&self).map(|obj| obj.move_by(x, y));
    }
    fn move_to(&self, arena: &mut Arena, x: f32, y: f32) {
        arena.get_mut(&self).map(|obj| obj.move_to(x, y));
    }
}

impl<Arena> GetPositionArena<Arena> for NodeIndex
where
    Arena: HasArena,
{
    fn position(&self, arena: &Arena) -> Option<Point> {
        let pos = arena.get(&self).map(|object| object.position());
        let origin = arena
            .get_parent(&self)
            .map(|parent| parent.position())
            .unwrap_or(point());

        pos.map(|pos| pos + origin)
    }
}

/// Create animation builder with given action, and return it
/// Builder allows chaining of commands for specifying animation properties
/// Builder generates Animation on drop
/// Animation::new(self.clone(), Action::Shift(by))
impl Animate for NodeIndex {
    fn shift(&self, by: Vector) -> TargetAction {
        TargetAction::new(self.clone(), Action::Shift { from: point(), by }, true)
    }
    fn move_to(&self, to: Point) -> TargetAction {
        TargetAction::new(self.clone(), Action::MoveTo { from: point(), to }, true)
    }
    fn to_edge(&self, direction: Vector) -> TargetAction {
        // Need to map direciton vector to internal enum
        // Direction vector is used to maintain consistency in API
        // Internally, enum makes it easier to compare
        let dir_enum: Direction;
        if direction == UP {
            dir_enum = Direction::Up;
        } else if direction == DOWN {
            dir_enum = Direction::Down;
        } else if direction == LEFT {
            dir_enum = Direction::Left;
        } else if direction == RIGHT {
            dir_enum = Direction::Right;
        } else {
            panic!("Invalid direction specified!! Direction must be one of UP/DOWN/LEFT/RIGHT");
        }
        TargetAction::new(
            self.clone(),
            Action::ToEdge {
                from: point(),
                to: point(),
                buffer: MED_SMALL_BUFF,
                direction: dir_enum,
            },
            true,
        )
    }
    fn show_creation(&self) -> TargetAction {
        TargetAction::new(self.clone(), Action::ShowCreation, true)
    }
}
