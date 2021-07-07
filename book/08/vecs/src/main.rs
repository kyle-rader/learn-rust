fn main() {
    println!("Hello, vecs!");

    // Now that we are pushing i32's below, we don't need this type annotation.
    // let mut evens: Vec<i32> = Vec::new();
    let mut evens: Vec<i32> = Vec::new();
    let mut odds = vec![1, 3, 5];

    evens.push(0);
    evens.push(2);
    evens.push(4);

    odds.push(7);
}
