use add_one;

fn main() {
    println!("Hello, adder!");
    println!("One more from {} is {}", 5, add_one::one_more(5));
    println!("Some more from {} is {}", 5, add_one::some_more(5));
}
