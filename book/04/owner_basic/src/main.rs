use rand::Rng;

fn print_string(name: &str, s: &str) {
    println!("{} is '{}' with length {}", name, s, s.len());
}

fn take_and_return(s: String) -> String {
    println!("we've taken '{}', and will now give it back.", s);
    s
}

// Mutable string, that we take ownership of and return the String.
fn add_stuff(mut s: String) -> String {
    let num = rand::thread_rng().gen_range(1..101);
    s.push_str(&num.to_string());
    s
}

// Mutable reference to a string, borrow it, but can mutate.
fn add_stuff_ref(s: &mut String) {
    let num = rand::thread_rng().gen_range(100..999);
    s.push_str(&num.to_string());
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
    // print_string("s1", s1); // Doesn't work, because s1 is now invalid!
    print_string("s2", &s2);

    println!("{}", s2);

    let mut s3 = s2.clone();
    s3.push_str("defg");
    print_string("s3", &s3);

    s3 = take_and_return(s3);
    print_string("s3", &s3);

    s3 = add_stuff(s3);
    print_string("s3", &s3);

    add_stuff_ref(&mut s3);
    print_string("s3", &s3);
}
