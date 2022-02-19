pub trait HelloMacro {
    fn hello_macro() -> String;
}

#[cfg(test)]
mod tests {
    use crate::HelloMacro;

    struct Foobar;

    impl HelloMacro for Foobar {
        fn hello_macro() -> String {
            format!("My name is Foobar")
        }
    }

    #[test]
    fn it_works() {
        assert_eq!(Foobar::hello_macro(), "My name is Foobar");
    }
}
