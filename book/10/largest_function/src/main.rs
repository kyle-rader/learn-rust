fn main() {
    let numbers = vec![34, 50, 12, 67, 89, 100, 94, 65];

    let mut largest = numbers[0];

    for n in numbers {
        if n > largest {
            largest = n;
        }
    }

    println!("largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
