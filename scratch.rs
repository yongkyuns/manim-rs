use std::convert::{From, Into};
use std::ops::Add;
#[derive(Debug, Copy, Clone)]
struct Point {
    x: f32,
    y: f32,
}
impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl SetPosition for Point {
    fn move_to(&self, x: f32, y: f32) -> Self {
        Self { x, y }
    }
    fn move_by(&self, x: f32, y: f32) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
        }
    }
    fn position(&self) -> Point {
        Self {
            x: self.x,
            y: self.y,
        }
    }
}

impl From<Point> for Primitive {
    fn from(point: Point) -> Self {
        Primitive::Point(point)
    }
}

impl Into<Option<Point>> for &mut Primitive {
    fn into(self) -> Option<Point> {
        match self {
            Primitive::Point(prim) => Some(*prim),
            _ => None,
        }
    }
}

trait SetPosition {
    fn move_to(&self, x: f32, y: f32) -> Self;
    fn move_by(&self, x: f32, y: f32) -> Self;
    fn position(&self) -> Point;
}

#[derive(Debug, Copy, Clone)]
struct Arrow {
    line: Line,
    arrow_width: f32,
    arrow_height: f32,
}
impl SetPosition for Arrow {
    fn position(&self) -> Point {
        self.line.position()
    }
    fn move_by(&self, x: f32, y: f32) -> Self {
        let line = self.line.move_by(x, y);
        Self {
            line,
            arrow_width: self.arrow_width,
            arrow_height: self.arrow_height,
        }
    }
    fn move_to(&self, x: f32, y: f32) -> Self {
        let line = self.line.move_to(x, y);
        Self {
            line,
            arrow_width: self.arrow_width,
            arrow_height: self.arrow_height,
        }
    }
}

impl From<Arrow> for Primitive {
    fn from(arrow: Arrow) -> Self {
        Primitive::Arrow(arrow)
    }
}
impl Into<Option<Arrow>> for Primitive {
    fn into(self) -> Option<Arrow> {
        match self {
            Primitive::Arrow(prim) => Some(prim),
            _ => None,
        }
    }
}
#[derive(Debug, Copy, Clone)]
struct Line {
    start: Point,
    end: Point,
}
impl SetPosition for Line {
    fn position(&self) -> Point {
        Point {
            x: (self.start.x + self.end.x) / 2.0,
            y: (self.start.y + self.end.y) / 2.0,
        }
    }
    fn move_by(&self, x: f32, y: f32) -> Self {
        Self {
            start: self.start + Point { x, y },
            end: self.end + Point { x, y },
        }
    }
    fn move_to(&self, x: f32, y: f32) -> Self {
        let dx = x - self.position().x;
        let dy = y - self.position().y;
        self.move_by(dx, dy)
    }
}
impl From<Line> for Primitive {
    fn from(line: Line) -> Self {
        Primitive::Line(line)
    }
}
impl Into<Option<Line>> for Primitive {
    fn into(self) -> Option<Line> {
        match self {
            Primitive::Line(prim) => Some(prim),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum Primitive {
    Arrow(Arrow),
    Line(Line),
    Point(Point),
}

fn main() {
    let mut point = Point { x: 0.0, y: 0.0 };
    point = point.move_to(10.0, 20.0).move_by(5.0, 5.0);
    let mut line = Line {
        start: point,
        end: Point { x: 0.0, y: 0.0 },
    };
    line = line.move_by(30.0, 30.0);
    let mut arrow = Arrow {
        line,
        arrow_width: 2.0,
        arrow_height: 4.0,
    };

    let mut shapes: Vec<Primitive> = Vec::new();
    shapes.push(Primitive::Line(line));
    shapes.push(Primitive::Point(point));
    shapes.push(Primitive::Arrow(arrow));

    for s in shapes.iter() {}

    // shapes = shapes
    //     .into_iter()
    //     .map(|x| x.move_by(100.0, 100.0))
    //     .collect();
    dbg!(&shapes);
}
