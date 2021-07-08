// fn largest_i32(list: &[i32]) -> Option<&i32> {
//     list.iter().reduce(|a, b| if a >= b { a } else { b })
// }

// fn largest_char(list: &[char]) -> Option<&char> {
//     list.iter().reduce(|a, b| if a >= b { a } else { b })
// }
use std::cmp::PartialOrd;

fn largest<T: PartialOrd>(list: &[T]) -> Option<&T> {
    list.iter().reduce(|a, b| if a >= b { a } else { b })
}

fn main() {
    let numbers = vec![34, 50, 12, 67, 89, 100, 94, 65];
    if let Some(largest) = largest(&numbers) {
        println!("largest number is {}", largest);
    }

    let numbers = vec![3102, 34, 6000, 89, 54, 2, 43, 8];
    if let Some(largest) = largest(&numbers) {
        println!("largest number is {}", largest);
    }

    let chars = ['a', 'b', 'c', 'i', 'j', 'k', 'z'];
    if let Some(largest) = largest(&chars) {
        println!("largest char is {}", largest);
    }

    let floats = [1.1, 4.5, 9.9, 8.5, 7.3];
    if let Some(largest) = largest(&floats) {
        println!("largest flots is {}", largest);
    }
}
