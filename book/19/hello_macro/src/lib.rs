pub trait HelloMacro {
    fn hello_macro();
}

#[cfg(test)]
mod tests {
    use crate::HelloMacro;

    struct Foobar;

    impl HelloMacro for Foobar {
        fn hello_macro() {
            println!("My name is Foobar")
        }
    }

    #[test]
    fn it_works() {
        Foobar::hello_macro();
        assert!(true);
    }
}
