fn find_largest(numbers: &[i32]) -> i32 {
    let mut largest = numbers[0];

    for &n in numbers {
        if n > largest {
            largest = n;
        }
    }
    largest
}

fn main() {
    let numbers = vec![34, 50, 12, 67, 89, 100, 94, 65];
    let largest = find_largest(&numbers);
    println!("largest number is {}", largest);

    let numbers = vec![3102, 34, 6000, 89, 54, 2, 43, 8];
    let largest = find_largest(&numbers);
    println!("largest number is {}", largest);
}
