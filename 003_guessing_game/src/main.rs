use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Guess the number! (It's {})", secret_number);
    println!("Input your guess.");

    loop {

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a whole number 😑");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small, try again."),
            Ordering::Greater => println!("Too big, try again."),
            Ordering::Equal => {
                println!("🚀 You win!");
                break;
            }
        }
    }
}
