use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

#[derive(HelloMacro)]
struct Syrup;

fn main() {
    Pancakes::hello_macro();
    println!("and");
    Syrup::hello_macro();
}
