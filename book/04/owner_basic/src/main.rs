fn print_string(name: &str, s: &str) {
    println!("{} is '{}' with length {}", name, s, s.len());
}

fn main() {
    {
        let mut s = String::from("Hello");
        s.push_str(" world!");
        println!("{}", s);
        // Implicitly called at end of block but we can call it ourself too.
        drop(s);
    }

    let s1 = String::from("I am string a");
    print_string("s1", &s1);

    let mut s2 = s1;
    s2.push_str("bc");
    // println!("s1 is '{}', with length {}", s1, s1.len()); // Doesn't work, because s1 is now invalid!
    print_string("s2", &s2);

    let mut s3 = s2.clone();
    s3.push_str("defg");
    print_string("s3", &s3);
}
