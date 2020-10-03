trait Bark {
    fn bark(&mut self);
}
struct Animal {
    is_barking: bool,
}
impl Bark for Animal {
    fn bark(&mut self) {
        println!("Bark Trait");
        self.bark();
        println!("Bark Trait");
    }
}
impl Animal {
    fn new() -> Self {
        Self { is_barking: false }
    }
    fn bark(&mut self) {
        println!("Woof!");
        self.is_barking = true;
        println!("Woof!");
    }
}
fn main() {
    let mut dog = Animal::new();
    // let mut dog: Box<dyn Bark> = Box::new(Animal::new());
    dog.bark();
}
