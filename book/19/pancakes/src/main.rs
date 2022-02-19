use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

#[derive(HelloMacro)]
struct Syrup;

fn main() {
    println!("{} and {}", Pancakes::hello_macro(), Syrup::hello_macro());
}
