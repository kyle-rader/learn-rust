fn main() {
    println!("Hello, panic!");

    // panic!("Crash and burn, but do clean up the stack first :)");

    let v = vec![1,2,3];
    v[10];
}
