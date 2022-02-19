use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

#[derive(HelloMacro)]
struct Syrup;

fn main() {
    println!("{}", Pancakes::hello_macro());
    println!("and");
    println!("{}", Syrup::hello_macro());
}
