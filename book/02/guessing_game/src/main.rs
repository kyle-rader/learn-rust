use rand::Rng;
use std::cmp::Ordering;
use std::io;

mod guess;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Guess the number! (It's {})", secret_number);
    println!("Input your guess.");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a whole number 😑");
                continue;
            }
        };

        let guess = match guess::Guess::new(guess) {
            Ok(g) => g,
            Err(msg) => {
                println!("{}", msg);
                continue;
            }
        };

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("{} is too small, try again.", guess.value()),
            Ordering::Greater => println!("{} is too big, try again.", guess.value()),
            Ordering::Equal => {
                println!("🚀 You win!");
                break;
            }
        }
    }
}
