#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("call on {:?}", self);
    }
}

fn main() {
    println!("Hello enums!");

    let home = IpAddr::V4(127, 0, 0, 1);
    let router = IpAddr::V4(192, 168, 1, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:?}", home);
    println!("{:?}", router);
    println!("{:?}", loopback);

    let m = Message::Write(String::from("hello message"));
    println!("m = {:?}", m);
    m.call();

    let m = Message::Move { x: 2, y: 5 };
    m.call();
}
