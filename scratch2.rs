// use std::error::Error;

#[derive(Debug)]
struct SomeError {}

fn do_something() -> Result<(), SomeError> {
    println!("Doing something here...");
    Ok(())
    // Err(SomeError {})
}

fn main() {
    do_something().unwrap();
}
