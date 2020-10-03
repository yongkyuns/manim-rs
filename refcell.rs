use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Object<'a> {
    point: &'a mut Point,
}
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}
struct Scene<'a> {
    objects: Vec<Rc<RefCell<Object<'a>>>>,
    point: Point,
}
impl<'a> Scene<'a> {
    fn add(&mut self, object: Rc<RefCell<Object<'a>>>) {
        self.objects.push(object);
    }
    fn add_point(&mut self, point: &mut Point) {
        self.point = point;
    }
    fn modify(&mut self) {
        for obj in self.objects.iter() {
            obj.borrow_mut().point.x += 10.0;
        }
    }
    fn print(&self) {
        for obj in self.objects.iter() {
            println!("{:?}", obj);
        }
    }
}

fn main() {
    let mut scene = Scene {
        objects: Vec::new(),
        point: Point { x: -10.0, y: -10.0 },
    };
    let mut obj = Rc::new(RefCell::new(Object {
        point: Point { x: 0.0, y: 0.0 },
    }));
    // scene.add(Rc::clone(&obj));
    scene.add(obj.clone());
    println!("{:?}", obj);
    scene.modify();
    println!("{:?}", obj);
    scene.add_point(&mut obj.clone().borrow_mut().point);
    obj.borrow_mut().point = Point { x: 20.0, y: 20.0 };
    scene.print();
}
