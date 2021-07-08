fn largest_i32(numbers: &[i32]) -> i32 {
    let mut largest = numbers[0];

    for &n in numbers {
        if n > largest {
            largest = n;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let numbers = vec![34, 50, 12, 67, 89, 100, 94, 65];
    let largest = largest_i32(&numbers);
    println!("largest number is {}", largest);

    let numbers = vec![3102, 34, 6000, 89, 54, 2, 43, 8];
    let largest = largest_i32(&numbers);
    println!("largest number is {}", largest);

    let chars = ['a', 'b', 'c', 'i', 'j', 'k', 'z'];
    let largest = largest_char(&chars);
    println!("largest char is {}", largest);
}
