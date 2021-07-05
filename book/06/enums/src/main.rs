#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    println!("Hello enums!");

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let router = IpAddr::V4(String::from("192.168.1.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:?}", home);
    println!("{:?}", router);
    println!("{:?}", loopback);
}
