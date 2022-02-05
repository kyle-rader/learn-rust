fn main() {
    println!("Hello, unsafe Rust!");
    
    let address = 0x012345usize;
    
    let r1 = address as *const i32;
    
    unsafe {
        println!("Aaaaand seg fault");
        println!("What is at {}? [{}]", address, *r1);
    }
}
