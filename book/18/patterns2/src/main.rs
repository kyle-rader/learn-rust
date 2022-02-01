fn main() {
    println!("Hello, patterns!");

    let x = 5;
    match x {
        1 => println!("one"),
        5 => println!("five"),
        _ => println!("other"),
    }

    let y = Some(5);
    match y {
        Some(10) => println!("Got 10"),
        Some(y) => println!("Got {}", y),
        _ => println!("Default case, x = {:?}", x),
    }
}
