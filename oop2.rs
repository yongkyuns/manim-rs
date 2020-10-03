trait Area {
    fn area(&self) -> f32;
}

#[derive(Debug)]
struct Rectangle {
    width: f32,
    height: f32,
}

impl Area for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }
}
#[derive(Debug)]
struct Triangle {
    width: f32,
    height: f32,
}

impl Area for Triangle {
    fn area(&self) -> f32 {
        self.width * self.height / 2.0
    }
}

enum Shapes {
    Rectangle(Rectangle),
    Triangle(Triangle),
}

fn main() {
    let rect = Rectangle {
        width: 10.0,
        height: 10.0,
    };
    let tri = Triangle {
        width: 10.0,
        height: 10.0,
    };
    println!("Rect area = {}", rect.area());
    println!("Triangle area = {}", tri.area());

    // let shapes = vec![&rect as &Area, &tri as &Area];
    // for s in shapes.iter() {
    //     println!("Area = {}", s.area());
    // }

    let shapes = vec![Shapes::Rectangle(rect), Shapes::Triangle(tri)];
    for s in shapes.iter() {
        match s {
            Shapes::Rectangle(rect) => println!("{}", rect.area()),
            Shapes::Triangle(tri) => println!("{}", tri.area()),
        }
    }
}
