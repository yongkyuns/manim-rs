use std::collections::HashMap;

fn main() {
    let mut hash: HashMap<usize, String> = HashMap::new();
    hash.insert(10, "First".to_owned());
    hash.insert(2, "Second".to_owned());
    println!("{:?}", hash.remove(&2));
    dbg!(&hash);

    let mut vec: Vec<Option<f32>> = Vec::new();
    vec.push(Some(60.0));
    vec.push(None);
    let el = vec.get_mut(2);
    dbg!(el);
    // if let Some(c) = vec.get_mut(2) {
    //     println!("Something!!");
    // }
}
