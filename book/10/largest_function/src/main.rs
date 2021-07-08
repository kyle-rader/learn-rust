fn largest_i32(list: &[i32]) -> Option<&i32> {
    list.iter().reduce(|a, b| if a >= b { a } else { b })
}

fn largest_char(list: &[char]) -> Option<&char> {
    list.iter().reduce(|a, b| if a >= b { a } else { b })
}

// Won't work yet as >= can't be used any just any T
// fn largest<T>(list: &[T]) -> Option<&T> {
//     list.iter().reduce(|a, b| if a >= b { a } else { b })
// }

fn main() {
    let numbers = vec![34, 50, 12, 67, 89, 100, 94, 65];
    if let Some(largest) = largest_i32(&numbers) {
        println!("largest number is {}", largest);
    }

    let numbers = vec![3102, 34, 6000, 89, 54, 2, 43, 8];
    if let Some(largest) = largest_i32(&numbers) {
        println!("largest number is {}", largest);
    }

    let chars = ['a', 'b', 'c', 'i', 'j', 'k', 'z'];
    if let Some(largest) = largest_char(&chars) {
        println!("largest char is {}", largest);
    }
}
