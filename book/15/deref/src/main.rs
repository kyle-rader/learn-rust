use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}

fn main() {
    println!("Hello, derefs!");
    hello("&str");
    let m = MyBox::new(String::from("But also Box<String>"));
    hello(&m);

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Box copies the value onto the heap
    let y = Box::new(x);
    assert_eq!(5, *y);

    let y = MyBox::new(5);
    assert_eq!(5, *y);
}
