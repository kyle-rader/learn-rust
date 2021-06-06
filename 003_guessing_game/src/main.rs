use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Guess the number! (It's {})", secret_number);
    println!("Input now.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a whole number 😑");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small."),
        Ordering::Greater => println!("Too big."),
        Ordering::Equal => println!("🚀 You win!"),
    }

}
