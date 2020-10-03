trait Show {
    fn print_values(&self);
}

impl Show for Vec<i32> {
    fn print_values(&self) {
        for e in self.iter() {
            println!("{}", e);
        }
    }
}
impl Show for &[i32] {
    fn print_values(&self) {
        for e in self.iter() {
            println!("{}", e);
        }
    }
}

fn main() {
    let a = vec![6, 7, 8, 9, 10, 11, 12];
    let slice = &a[0..=2];
    // dbg!(&a);
    // dbg!(&slice);
    // let b: Vec<i32> = a.iter().take(2).collect();
    let mut it = a.iter().skip(2).take(2);
    assert_eq!(it.next(), Some(&8));
    assert_eq!(it.next(), Some(&9));
    assert_eq!(it.next(), None);

    // a.print_values();
    // slice.print_values();
}
