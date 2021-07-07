use std::collections::HashMap;
use std::io;

fn averages(numbers: &Vec<f64>) -> HashMap<&str, f64> {
    let mut sum = 0.0;
    for i in numbers {
        sum += i;
    }

    vec![
        ("median", 0.0),
        ("mean", sum / numbers.len() as f64),
        ("mode", 0.0),
    ]
    .iter()
    .cloned()
    .collect()
}

fn main() {
    println!("Please enter your numbers:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    let numbers: Vec<f64> = input
        // get tokens Iter of &str
        .split_whitespace()
        // parse and filter to only those that parse as f64
        //                 👇 this type annotation isn't required
        .filter_map(|word: &str| word.parse::<f64>().ok())
        // filter away NAN, and [NEG]INFITE values
        .filter(|i| i.is_finite())
        // collect into Vec<f64>
        .collect();

    let summary = averages(&numbers);
    println!("{:#?}", summary);
}
