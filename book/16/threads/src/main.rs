use std::thread;
use std::time::Duration;

fn main() {
    let child_handle = thread::spawn(|| {
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

    if let Ok(_) = child_handle.join() {
        println!("Joined child thread!");
    }
    else {
        println!("🤮 Failed to join child thread!");
    }
}
