fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    // for loop takes ownership of iterator so we cannot reuse v1_iter here
    // assert_eq!(v1_iter.next(), None);

    println!("sum is {}", v1.iter().sum::<i32>());
}
