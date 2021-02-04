use crate::animation::PathCompletion;
use crate::appearance::Visibility;
use crate::draw::Draw;
use crate::geom::{GetPosition, Point, SetPosition};

use nannou;

pub use self::circle::Circle;
pub use self::rectangle::Rectangle;
pub use self::text::Text;

pub mod circle;
pub mod rectangle;
pub mod text;

#[derive(Debug, PartialEq)]
pub enum Object {
    Circle(Circle),
    Rectangle(Rectangle),
    Text(Text),
}

impl SetPosition for Object {
    fn position_mut(&mut self) -> &mut Point {
        match self {
            Object::Circle(c) => SetPosition::position_mut(c),
            Object::Rectangle(r) => SetPosition::position_mut(r),
            Object::Text(t) => SetPosition::position_mut(t),
        }
    }
}

impl GetPosition for Object {
    fn position(&self) -> Point {
        match self {
            Object::Circle(c) => GetPosition::position(c),
            Object::Rectangle(r) => GetPosition::position(r),
            Object::Text(t) => GetPosition::position(t),
        }
    }
}

impl PathCompletion for Object {
    fn completion(&self) -> f32 {
        match self {
            Object::Circle(c) => c.completion(),
            Object::Rectangle(r) => r.completion(),
            Object::Text(t) => t.completion(),
        }
    }
    fn set_completion(&mut self, completion: f32) {
        match self {
            Object::Circle(c) => c.set_completion(completion),
            Object::Rectangle(r) => r.set_completion(completion),
            Object::Text(t) => t.set_completion(completion),
        }
    }
}

impl Draw for Object {
    fn draw(&self, draw: nannou::Draw) {
        match self {
            Object::Circle(c) => c.draw(draw),
            Object::Rectangle(r) => r.draw(draw),
            Object::Text(t) => t.draw(draw),
        }
    }
}

impl Visibility for Object {
    fn visible_mut(&mut self) -> &mut bool {
        match self {
            Object::Circle(c) => Visibility::visible_mut(c),
            Object::Rectangle(r) => Visibility::visible_mut(r),
            Object::Text(t) => Visibility::visible_mut(t),
        }
    }
    fn is_visible(&self) -> bool {
        match self {
            Object::Circle(c) => c.is_visible(),
            Object::Rectangle(r) => r.is_visible(),
            Object::Text(t) => t.is_visible(),
        }
    }
}

impl From<Circle> for Object {
    fn from(c: Circle) -> Self {
        Object::Circle(c)
    }
}

impl From<Rectangle> for Object {
    fn from(r: Rectangle) -> Self {
        Object::Rectangle(r)
    }
}

impl From<Text> for Object {
    fn from(t: Text) -> Self {
        Object::Text(t)
    }
}

// impl PathCompletion for NodeIndex {
//     fn completion(&self) -> f32 {
//         PathCompletion::completion(&*self.clone().borrow_mut())
//     }
//     fn set_completion(&mut self, completion: f32) {
//         self.borrow_mut().set_completion(completion);
//     }
// }

// impl SetPosition for NodeIndex {
//     fn position(&self) -> Point {
//         SetPosition::position(&*self.clone().borrow_mut())
//     }
//     fn set_position(&mut self, to: Point) {
//         self.borrow_mut().set_position(to);
//     }
// }

// impl Draw for NodeIndex {
//     fn draw(&self, draw: nannou::Draw) {
//         self.borrow().draw(draw);
//     }
// }
