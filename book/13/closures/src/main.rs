use std::thread;
use std::time::Duration;

fn sim_calc(i: u32) -> u32 {
    println!("calculating...");
    thread::sleep(Duration::from_secs(1));
    i
}

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            sim_calc(intensity)
        );
        println!(
            "Next, do {} situps!",
            sim_calc(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                sim_calc(intensity)
            );
        }
    }
}

fn main() {
    println!("Hello, closures!");

    let sim_user_val = 10;
    let sim_random = 4;

    generate_workout(sim_user_val, sim_random);
}
