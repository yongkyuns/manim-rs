use super::{Arena, HasArena};
use crate::animation::PathCompletion;
use crate::appearance::Visibility;
use crate::draw::Draw;
use crate::geom::{Dimension, GetDimension, SetDimension};
use crate::geom::{GetOrientation, GetPosition, Point, SetOrientation, SetPosition};
use crate::object::Object as InnerObject;
use crate::scene::Scene;

use generational_arena::Index;

// pub type Object = Node; // Treat Node like an object
// pub type Id = NodeIndex; // Short-hand for readability

pub use Node as Object; // Treat Node like an object
pub use NodeIndex as Id; // Short-hand for readability

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

impl GetOrientation for Node {
    fn orientation(&self) -> f32 {
        GetOrientation::orientation(&self.inner)
    }
}

impl SetOrientation for Node {
    fn orientation_mut(&mut self) -> &mut f32 {
        SetOrientation::orientation_mut(&mut self.inner)
    }
}

impl GetDimension for Node {
    fn dimension(&self) -> &Dimension {
        GetDimension::dimension(&self.inner)
    }
}

impl SetDimension for Node {
    fn dimension_mut(&mut self) -> &mut Dimension {
        SetDimension::dimension_mut(&mut self.inner)
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
    fn delete(&mut self, id: NodeIndex);
}

impl NodeArena for Arena<Node> {
    fn add(&mut self, node: Node) -> NodeIndex {
        NodeIndex(self.insert(node))
    }
    fn delete(&mut self, id: NodeIndex) {
        self.remove(id.0);
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

// impl Into<Index> for NodeIndex {
//     // fn from(index: Index) -> Self {
//     //     Self(index)
//     // }
//     fn into(self) -> Index {
//         self.0
//     }
// }
impl From<NodeIndex> for Index {
    fn from(ni: NodeIndex) -> Self {
        ni.0
    }
}

// impl From<Index> for NodeIndex {
//     fn from(index: Index) -> Self {
//         Self(index)
//     }
// }
