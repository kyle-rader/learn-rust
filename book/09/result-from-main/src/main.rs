use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world! We're going to try to read a file!");
    let content = fs::read_to_string("hello.txt")?;
    println!("{}", content);
    Ok(())
}
