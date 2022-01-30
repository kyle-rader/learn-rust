use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    println!("Hello, channels!");

    let (tx, rx) = mpsc::channel();

    let tx2 = tx.clone();
    thread::spawn(move || {
        for i in 0..10 {
            let msg = format!("msg {} from tx2", i);
            tx2.send(msg).expect(&format!("Failed to send {} from tx2", i)[..]);
            thread::sleep(Duration::from_millis(2));
        }
    });

    thread::spawn(move || {
        let val = String::from("hi there");
        tx.send(val).expect("Failed to send!");

        let vals = vec!["one", "two", "three", "four", "five"];
        for val in vals {
            tx.send(String::from(val)).expect(&format!("Failed to send {}", val)[..]);
            thread::sleep(Duration::from_millis(2));
        }
    });

    for msg in rx {
        println!("Got {:?}", msg);
    }
}
