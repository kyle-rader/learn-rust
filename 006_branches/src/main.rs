use std::io;

fn main() {
    let mut num = String::new();

    println!("Enter a number >");

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");

    let num: u32 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            panic!("Please type a whole number 😑");
        }
    };

    let rounded = num + (100 - (num % 100));

    println!("num: {} bucket: {}", num, rounded);
}
