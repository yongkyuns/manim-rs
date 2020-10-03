use std::cell::RefCell;
use std::cell::{Ref, RefMut};
use std::rc::Rc;

#[derive(Debug)]
struct Parent {
    child: RefCell<Child>,
}
#[derive(Debug, Clone)]
struct Child {
    name: String,
}

fn modify(parent: &Parent) {
    parent.child.borrow_mut().name = "Hey!".to_owned();
}
fn modify_child(child: &RefCell<Child>) {
    child.borrow_mut().name = "Wow".to_owned();
}

fn main() {
    let name = "Hello".to_owned();
    let child = Child { name };
    let parent = Parent {
        child: RefCell::new(child),
    };

    // let closure = || {
    //     println!("{:?}", parent);
    // };
    // parent.child.name = "You".to_owned();

    // closure();

    // let child_mut = parent.child.borrow_mut();
    // child_mut.clone().name = "Weird!".to_owned();

    parent.child.borrow_mut().clone().name = "Weird!".to_owned();
    println!("Parent = {:?}", &parent);

    // modify_child(parent.child.clone());
    // child.name = "Whoa".to_owned();

    // parent.child.member.text = "Hey".to_owned();
    // val = 20.0;

    // println!("Parent = {:?}", &parent);
}
