fn check(password: &str) -> bool {
    // Must have at least
    // 1 lower case char
    // 1 upper case char
    // 1 special char (!@#$%^&*-_)
    // 1 digit
    let mut lower = false;
    let mut upper = false;
    let mut special = false;
    let mut digit = false;
    for (i, c) in password.chars().enumerate() {
        if !lower && c.is_lowercase() {
            lower = true;
        }
        if !upper && c.is_uppercase() {
            upper = true;
        }
        if !special && c.is_ascii_punctuation() {
            special = true;
        }
        if !digit && c.is_digit(10) {
            digit = true;
        }
    }
    lower && upper && special && digit
}

fn check_print(password: &str) {
    match check(password) {
        true => println!("{} is strong enough", password),
        false => println!("{} is too weak", password),
    }
}

fn main() {
    check_print("$ThisIsGood123");
    check_print("badPassword$");
}
