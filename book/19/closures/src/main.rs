fn silly_to_string(x: &u32) -> String {
    match x {
        0 => String::from("nothing"),
        1..=10 => String::from("little"),
        11..=100 => String::from("medium"),
        _ => String::from("big"),
    }
}

fn main() {
    println!("Hello, world!");
    let numbers: Vec<u32> = vec![0, 240, 34, 5, 8, 129, 0, 5];
    let items: Vec<String> = numbers
        .iter()
        .map(silly_to_string)
        .collect();

    for num in items {
        println!("{}", num);        
    }
}
