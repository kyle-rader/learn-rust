use std::thread;
use std::time::Duration;

fn print_n_sleep(name: &str, n: usize) {
    for i in 1..n {
        println!("{} : {}", name, i);
        thread::sleep(Duration::from_millis(5));
    }
    println!("{} done.🏁", name);
}

fn main() {
    let child_handle = thread::spawn(|| {
        print_n_sleep("Child", 10);
    });

    print_n_sleep("Parent", 5);

    if let Ok(_) = child_handle.join() {
        println!("Joined child thread!");
    } else {
        println!("🤮 Failed to join child thread!");
    }

    println!("Using move closures with threads:");

    let v = vec![1, 1, 2, 3, 5, 8];
    let child_handle = thread::spawn(move || {
        println!("Our numbers are {:?}", v);
    });

    child_handle.join().unwrap();
}
