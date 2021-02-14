use crate::animation::PathCompletion;
use crate::appearance::{GetOpacity, Opacity, SetOpacity};
use crate::draw::Draw;
use crate::geom::{Dimension, GetDimension, SetDimension};
use crate::geom::{GetOrientation, GetPosition, Point, SetOrientation, SetPosition};

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
            Object::Circle(o) => SetPosition::position_mut(o),
            Object::Rectangle(o) => SetPosition::position_mut(o),
            Object::Text(o) => SetPosition::position_mut(o),
        }
    }
}

impl GetPosition for Object {
    fn position(&self) -> Point {
        match self {
            Object::Circle(o) => GetPosition::position(o),
            Object::Rectangle(o) => GetPosition::position(o),
            Object::Text(o) => GetPosition::position(o),
        }
    }
}

impl SetOrientation for Object {
    fn orientation_mut(&mut self) -> &mut f32 {
        match self {
            Object::Circle(_) => panic!("Circle cannot rotate... yet."),
            Object::Rectangle(o) => SetOrientation::orientation_mut(o),
            Object::Text(o) => SetOrientation::orientation_mut(o),
        }
    }
}

impl GetOrientation for Object {
    fn orientation(&self) -> f32 {
        match self {
            Object::Circle(_) => panic!("Circle does not have angle... yet."),
            Object::Rectangle(o) => GetOrientation::orientation(o),
            Object::Text(o) => GetOrientation::orientation(o),
        }
    }
}

impl GetDimension for Object {
    fn dimension(&self) -> &Dimension {
        match self {
            Object::Circle(o) => GetDimension::dimension(o),
            Object::Rectangle(o) => GetDimension::dimension(o),
            Object::Text(o) => GetDimension::dimension(o),
        }
    }
}

impl SetDimension for Object {
    fn dimension_mut(&mut self) -> &mut Dimension {
        match self {
            Object::Circle(o) => SetDimension::dimension_mut(o),
            Object::Rectangle(o) => SetDimension::dimension_mut(o),
            Object::Text(o) => SetDimension::dimension_mut(o),
        }
    }
    fn set_height(&mut self, height: f32) {
        match self {
            Object::Circle(o) => SetDimension::set_height(o, height),
            Object::Rectangle(o) => SetDimension::set_height(o, height),
            Object::Text(o) => SetDimension::set_height(o, height),
        }
    }
    fn set_width(&mut self, width: f32) {
        match self {
            Object::Circle(o) => SetDimension::set_width(o, width),
            Object::Rectangle(o) => SetDimension::set_width(o, width),
            Object::Text(o) => SetDimension::set_width(o, width),
        }
    }
    fn set_size(&mut self, size: Dimension) {
        match self {
            Object::Circle(o) => SetDimension::set_size(o, size),
            Object::Rectangle(o) => SetDimension::set_size(o, size),
            Object::Text(o) => SetDimension::set_size(o, size),
        }
    }
}

impl PathCompletion for Object {
    fn completion(&self) -> f32 {
        match self {
            Object::Circle(o) => o.completion(),
            Object::Rectangle(o) => o.completion(),
            Object::Text(o) => o.completion(),
        }
    }
    fn set_completion(&mut self, completion: f32) {
        match self {
            Object::Circle(o) => o.set_completion(completion),
            Object::Rectangle(o) => o.set_completion(completion),
            Object::Text(o) => o.set_completion(completion),
        }
    }
}

impl Draw for Object {
    fn draw(&self, draw: nannou::Draw) {
        match self {
            Object::Circle(o) => o.draw(draw),
            Object::Rectangle(o) => o.draw(draw),
            Object::Text(o) => o.draw(draw),
        }
    }
}

impl GetOpacity for Object {
    fn opacity(&self) -> f32 {
        match self {
            Object::Circle(o) => GetOpacity::opacity(o),
            Object::Rectangle(o) => GetOpacity::opacity(o),
            Object::Text(o) => GetOpacity::opacity(o),
        }
    }
    fn is_visible(&self) -> bool {
        match self {
            Object::Circle(o) => GetOpacity::is_visible(o),
            Object::Rectangle(o) => GetOpacity::is_visible(o),
            Object::Text(o) => GetOpacity::is_visible(o),
        }
    }
}

impl SetOpacity for Object {
    fn opacity_mut(&mut self) -> &mut Opacity {
        match self {
            Object::Circle(o) => SetOpacity::opacity_mut(o),
            Object::Rectangle(o) => SetOpacity::opacity_mut(o),
            Object::Text(o) => SetOpacity::opacity_mut(o),
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
