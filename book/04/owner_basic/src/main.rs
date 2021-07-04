fn main() {
    {
        let mut s = String::from("Hello");
        s.push_str(" world!");
        println!("{}", s);
        // Implicitly called at end of block but we can call it ourself too.
        drop(s);
    }

    let s = String::from("Goodbye!!");
    println!("{}", s);
}
