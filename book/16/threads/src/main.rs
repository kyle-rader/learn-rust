use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("child : {}", i);
            thread::sleep(Duration::from_millis(5));
        }
        println!("child done.🏁");
    });

    for i in 1..5 {
        println!("parent: {}", i);
        thread::sleep(Duration::from_millis(5));
    }
    println!("parent done.🏁");
}
