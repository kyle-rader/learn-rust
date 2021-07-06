fn reverse(input: &str) -> String {
    let mut temp = String::new();
    let mut result = String::new();

    for c in input.chars() {
        if c == ' ' {
            result.push_str(&temp);
            temp.clear();
            result.push(c);
        } else {
            temp.insert(0, c);
        }
    }
    result.push_str(&temp);
    result
}

fn print_reverse(input: &str) {
    println!("{}\n{}", input, reverse(input));
}

fn main() {
    print_reverse("Hello, world!");
    print_reverse("Many thanks to you sir!");
}
