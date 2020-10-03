#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}
type Position = Point;
impl SetPosition for Position {
    fn position(&mut self) -> &mut Position {
        self
    }
}
trait SetPose: SetPosition + SetOrientation {
    fn position(&mut self) -> &mut Position {
        SetPosition::position(self)
    }
    fn orientation(&mut self) -> &mut f32 {
        SetOrientation::orientation(self)
    }
}
trait SetPosition {
    fn goto(&mut self, x: f32, y: f32) {
        self.position().x = x;
        self.position().y = y;
    }
    fn position(&mut self) -> &mut Position;
}

//////////////////////////////////////////////////////////
impl SetOrientation for f32 {
    fn orientation(&mut self) -> &mut f32 {
        self
    }
}
trait SetOrientation {
    fn look_at(&mut self, angle: f32) {
        *self.orientation() = angle;
    }
    fn orientation(&mut self) -> &mut f32;
}

//////////////////////////////////////////////////////////

#[derive(Debug)]
struct Player {
    position: Position,
    orientation: f32,
}
impl Player {
    fn new() -> Self {
        Self {
            position: Position { x: 0.0, y: 0.0 },
            orientation: 0.0,
        }
    }
}
impl SetPosition for Player {
    fn position(&mut self) -> &mut Position {
        SetPosition::position(&mut self.position)
    }
}
impl SetOrientation for Player {
    fn orientation(&mut self) -> &mut f32 {
        SetOrientation::orientation(&mut self.orientation)
    }
}
// impl<Player: SetOrientation + SetPosition> SetPose for Player {
//     fn position(&mut self) -> &mut Position {
//         SetPosition::position(&mut self.position)
//     }
//     fn orientation(&mut self) -> &mut f32 {
//         SetOrientation::orientation(&mut self.orientation)
//     }
// }
//////////////////////////////////////////////////////////

#[derive(Debug)]
struct Admin {
    player: Player,
}
impl Admin {
    fn new() -> Self {
        Self {
            player: Player {
                position: Position { x: 0.0, y: 0.0 },
                orientation: 0.0,
            },
        }
    }
}
impl SetPosition for Admin {
    fn position(&mut self) -> &mut Position {
        SetPosition::position(&mut self.player)
    }
}
impl SetOrientation for Admin {
    fn orientation(&mut self) -> &mut f32 {
        SetOrientation::orientation(&mut self.player)
    }
}
//////////////////////////////////////////////////////////
#[derive(Debug)]
struct Monster {
    position: Position,
    orientation: f32,
}
impl Monster {
    fn new() -> Self {
        Self {
            position: Position { x: 0.0, y: 0.0 },
            orientation: 0.0,
        }
    }
}
impl SetPosition for Monster {
    fn position(&mut self) -> &mut Position {
        SetPosition::position(&mut self.position)
    }
}
impl SetOrientation for Monster {
    fn orientation(&mut self) -> &mut f32 {
        SetOrientation::orientation(&mut self.orientation)
    }
}
//////////////////////////////////////////////////////////

use std::ops::{Div, Mul};
trait SetDimension<T = f32> {
    fn width(&self) -> T;
    fn set_width(&mut self, width: T);
    fn height(&self) -> T;
    fn set_height(&mut self, height: T);
}

// trait Rectangle<T = f32>: SetDimension<T> + Sized
// where
//     T: Mul<Output = T> + Copy,
// {
//     fn area(&self) -> T {
//         self.height() * self.width()
//     }
// }

// trait Triangle<T = f32>: SetDimension<T> + Sized
// where
//     T: From<f32> + Mul<Output = T> + Div<Output = T> + Copy,
// {
//     fn area(&self) -> T {
//         self.height() * self.width() / T::from(2.0)
//     }
// }

#[derive(Debug)]
struct Rectangle<T = f32> {
    size: Dimension<T>,
}
impl<T> Rectangle<T> {
    fn area(&self) -> T
    where
        T: Mul<Output = T> + Copy,
    {
        self.size.width() * self.size.height()
    }
}
impl<T: Copy> SetDimension<T> for Rectangle<T> {
    fn width(&self) -> T {
        SetDimension::width(&self.size)
    }
    fn set_width(&mut self, width: T) {
        SetDimension::set_width(&mut self.size, width);
    }
    fn height(&self) -> T {
        SetDimension::height(&self.size)
    }
    fn set_height(&mut self, height: T) {
        SetDimension::set_height(&mut self.size, height);
    }
}
#[derive(Debug)]
struct Triangle<T = f32> {
    size: Dimension<T>,
}
impl<T> Triangle<T> {
    fn area(&self) -> T
    where
        T: From<f32> + Mul<Output = T> + Div<Output = T> + Copy,
    {
        self.size.width() * self.size.height() / T::from(2.0)
    }
}
impl<T: Copy> SetDimension<T> for Triangle<T> {
    fn width(&self) -> T {
        SetDimension::width(&self.size)
    }
    fn set_width(&mut self, width: T) {
        SetDimension::set_width(&mut self.size, width);
    }
    fn height(&self) -> T {
        SetDimension::height(&self.size)
    }
    fn set_height(&mut self, height: T) {
        SetDimension::set_height(&mut self.size, height);
    }
}
#[derive(Debug)]
struct Dimension<T> {
    width: T,
    height: T,
}
// impl<T> Rectangle<T> for Dimension<T> where T: Mul<Output = T> + Copy {}
// impl<T> Triangle<T> for Dimension<T> where T: From<f32> + Mul<Output = T> + Div<Output = T> + Copy {}
impl<T: Copy> SetDimension<T> for Dimension<T> {
    fn height(&self) -> T {
        self.height
    }
    fn set_height(&mut self, height: T) {
        self.height = height
    }
    fn width(&self) -> T {
        self.width
    }
    fn set_width(&mut self, width: T) {
        self.width = width
    }
}
#[derive(Debug)]
enum Polygon<T, S> {
    Rectangle(Rectangle<T>),
    Triangle(Triangle<S>),
}

//////////////////////////////////////////////////////////

fn main() {
    // let rect: Box<dyn Rectangle> = Box::new(Dimension {
    //     width: 10.0,
    //     height: 10.0,
    // });
    // let rect: &mut Rectangle = &mut Dimension {
    //     width: 10i32,
    //     height: 10i32,
    // };
    // let tri: &Triangle = &Dimension {
    //     width: 10.0,
    //     height: 10.0,
    // };
    let mut rect = Rectangle {
        size: Dimension {
            width: 10i32,
            height: 10i32,
        },
    };
    rect.set_height(20i32);
    let mut tri = Triangle {
        size: Dimension {
            width: 10.0,
            height: 10.0,
        },
    };
    tri.set_width(5.0);
    println!("Rectangle Area = {}", rect.area());
    println!("Triangle Area = {}", tri.area());

    let shapes = vec![Polygon::Rectangle(rect), Polygon::Triangle(tri)];
    dbg!(&shapes);
    // let mut player = Player::new();
    // let mut monster = Monster::new();
    // let mut admin = Admin::new();

    // player.goto(30.0, 30.0);
    // player.look_at(10.0);
    // monster.goto(20.0, 20.0);
    // monster.look_at(10.0);
    // admin.goto(40.0, 40.0);
    // admin.look_at(10.0);

    // dbg!(&player);
    // dbg!(&monster);
    // dbg!(&admin);

    let frog = Frog { name: "Frogger" };
    let toad = Toad { name: "Toady" };
    {
        let animals = vec![&frog as &Amphibian, &toad as &Amphibian];

        for animal in animals.iter() {
            animal.hop();
        }
    }
}
trait Amphibian {
    fn hop(&self);
}

struct Frog {
    name: &'static str,
}

struct Toad {
    name: &'static str,
}

impl Amphibian for Frog {
    fn hop(&self) {
        println!("Hopping frog {}", self.name);
    }
}

impl Amphibian for Toad {
    fn hop(&self) {
        println!("Hopping toad {}", self.name);
    }
}
