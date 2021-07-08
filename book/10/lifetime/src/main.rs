use std::fmt::Display;

fn main() {
    println!("Hello, lifetimes!");

    // Code we want to work
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let result = longest_with_alert(&string1, &string2, "checking who is longer!");
    print!("It's {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_with_alert<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Alert! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
