use std::{fmt, ops::{Deref, DerefMut}};

struct Wrapper(Vec<String>);

impl Deref for Wrapper {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Wrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(" - "))
    }
}

fn main() {
    let mut w = Wrapper(vec![String::from("hello"), String::from("newtype"), String::from("pattern")]);

    println!("Hello, World!");
    println!("w = {}", w);
    
    w.push(String::from("new stuff!"));
    println!("w = {}", w);
}
