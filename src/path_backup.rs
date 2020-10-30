#[derive(Debug, PartialEq, PartialOrd)]
struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, PartialOrd)]
enum Action {
    Line,
    Move,
    Cubic,
    Quadratic,
}
#[derive(Debug)]
struct Path {
    points: Box<[Point]>,
    actions: Box<[Action]>,
}

#[derive(Debug)]
struct PathSlice<'l> {
    points: &'l [Point],
    actions: &'l [Action],
}
#[derive(Debug)]
struct Builder {
    points: Vec<Point>,
    actions: Vec<Action>,
}

#[allow(dead_code)]
impl Builder {
    fn new() -> Self {
        Self {
            points: vec![],
            actions: vec![],
        }
    }
    fn move_to(&mut self, to: Point) -> bool {
        self.points.push(to);
        self.actions.push(Action::Move);
        true
    }
    fn line_to(&mut self, to: Point) -> bool {
        self.points.push(to);
        self.actions.push(Action::Line);
        true
    }
    pub fn quadratic_bezier_to(&mut self, ctrl: Point, to: Point) -> bool {
        self.points.push(ctrl);
        self.points.push(to);
        self.actions.push(Action::Quadratic);
        true
    }

    pub fn cubic_bezier_to(&mut self, ctrl1: Point, ctrl2: Point, to: Point) -> bool {
        self.points.push(ctrl1);
        self.points.push(ctrl2);
        self.points.push(to);
        self.actions.push(Action::Cubic);
        true
    }

    fn build(self) -> Path {
        Path {
            points: self.points.into_boxed_slice(),
            actions: self.actions.into_boxed_slice(),
        }
    }
}

trait Build {
    type PathType;
    fn build(self) -> Self::PathType;
    fn build_and_reset(&mut self) -> Self::PathType;
}

impl Build for Builder {
    type PathType = Path;
    fn build(self) -> Self::PathType {
        self.build()
    }
    fn build_and_reset(&mut self) -> Self::PathType {
        Path {
            points: std::mem::replace(&mut self.points, Vec::new()).into_boxed_slice(),
            actions: std::mem::replace(&mut self.actions, Vec::new()).into_boxed_slice(),
        }
    }
}
trait FlatPathBuilder {
    fn move_to(&mut self, to: Point);
    fn line_to(&mut self, to: Point);
}
impl FlatPathBuilder for Builder {
    fn move_to(&mut self, to: Point) {
        self.move_to(to);
    }
    fn line_to(&mut self, to: Point) {
        self.line_to(to);
    }
}
#[allow(dead_code)]
trait PathBuilder: FlatPathBuilder {
    fn quadratic_bezier_to(&mut self, ctrl: Point, to: Point);
    fn cubic_bezier_to(&mut self, ctrl1: Point, ctrl2: Point, to: Point);

    fn path_event(&mut self, event: PathEvent) {
        match event {
            PathEvent::Begin { at } => {
                self.move_to(at);
            }
            PathEvent::Line { to, .. } => {
                self.line_to(to);
            }
            PathEvent::Quadratic { ctrl, to, .. } => {
                self.quadratic_bezier_to(ctrl, to);
            }
            PathEvent::Cubic {
                ctrl1, ctrl2, to, ..
            } => {
                self.cubic_bezier_to(ctrl1, ctrl2, to);
            }
        }
    }
}
impl PathBuilder for Builder {
    fn quadratic_bezier_to(&mut self, ctrl: Point, to: Point) {
        self.quadratic_bezier_to(ctrl, to);
    }

    fn cubic_bezier_to(&mut self, ctrl1: Point, ctrl2: Point, to: Point) {
        self.cubic_bezier_to(ctrl1, ctrl2, to);
    }
}
#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub enum Event<Endpoint, ControlPoint> {
    Begin {
        at: Endpoint,
    },
    Line {
        from: Endpoint,
        to: Endpoint,
    },
    Quadratic {
        from: Endpoint,
        ctrl: ControlPoint,
        to: Endpoint,
    },
    Cubic {
        from: Endpoint,
        ctrl1: ControlPoint,
        ctrl2: ControlPoint,
        to: Endpoint,
    },
}

/// A path event representing endpoints and control points as positions.
#[allow(dead_code)]
type PathEvent = Event<Point, Point>;

#[cfg(test)]
mod tests {
    // use super::*;
    use nannou::lyon::algorithms::walk::{PathWalker, RegularPattern};
    use nannou::lyon::math::*;
    use nannou::lyon::path::builder::FlatPathBuilder;
    // #[test]
    // fn add_move_and_line() {
    //     let mut builder = Builder::new();
    //     builder.move_to(Point { x: 1.0, y: 1.0 });
    //     builder.line_to(Point { x: 2.0, y: 2.0 });
    //     let path = builder.build();
    //     // let path = builder.build_and_reset();

    //     let mut points_it = path.points.iter();
    //     let mut action_it = path.actions.iter();

    //     //Check that points and actions are correctly inserted
    //     assert_eq!(points_it.next().unwrap(), &Point { x: 1.0, y: 1.0 });
    //     assert_eq!(action_it.next().unwrap(), &Action::Move);

    //     assert_eq!(points_it.next().unwrap(), &Point { x: 2.0, y: 2.0 });
    //     assert_eq!(action_it.next().unwrap(), &Action::Line);

    //     //Check reset
    // }

    #[test]
    fn test_walk() {
        let expected = [
            (point(0.0, 0.0), vector(1.0, 0.0), 0.0),
            (point(2.0, 0.0), vector(1.0, 0.0), 2.0),
            (point(4.0, 0.0), vector(1.0, 0.0), 4.0),
            (point(6.0, 0.0), vector(1.0, 0.0), 6.0),
            (point(6.0, 2.0), vector(0.0, 1.0), 8.0),
            (point(6.0, 4.0), vector(0.0, 1.0), 10.0),
            (point(6.0, 6.0), vector(0.0, 1.0), 12.0),
            (point(4.0, 6.0), vector(-1.0, 0.0), 14.0),
            (point(2.0, 6.0), vector(-1.0, 0.0), 16.0),
            (point(0.0, 6.0), vector(-1.0, 0.0), 18.0),
            (point(0.0, 4.0), vector(0.0, -1.0), 20.0),
            (point(0.0, 2.0), vector(0.0, -1.0), 22.0),
            (point(0.0, 0.0), vector(0.0, -1.0), 24.0),
        ];

        let mut i = 0;
        let mut pattern = RegularPattern {
            interval: 2.0,
            callback: |pos, n, d| {
                println!("p:{:?} n:{:?} d:{:?}", pos, n, d);
                assert_eq!(pos, expected[i].0);
                assert_eq!(n, expected[i].1);
                assert_eq!(d, expected[i].2);
                i += 1;
                true
            },
        };

        let mut walker = PathWalker::new(0.0, &mut pattern);

        walker.move_to(point(0.0, 0.0));
        walker.line_to(point(6.0, 0.0));
        walker.line_to(point(6.0, 6.0));
        walker.line_to(point(0.0, 6.0));
        walker.close();
    }
}
