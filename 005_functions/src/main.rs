fn main() {
    println!("Hello, world!");
    another_fn(42);
    let x = 21;
    println!("{} doubled is {}", x, double(x));
}

fn another_fn(x: i32) {
    println!("I'm another_fn. I was given {}", x);
}

fn double(x: i32) -> i32 {
    x * 2
}
