fn main() {
    {
        let mut s = String::from("Hello");
        s.push_str(" world!");
        println!("{}", s);
        // Implicitly called at end of block but we can call it ourself too.
        drop(s);
    }

    let s1 = String::from("I am string a");
    println!("s1 is '{}', with length {}", s1, s1.len());

    let mut s2 = s1;
    s2.push_str("bc");
    // println!("s1 is '{}', with length {}", s1, s1.len()); // Doesn't work, because s1 is now invalid!
    println!("s2 is '{}', with length {}", s2, s2.len());
}
