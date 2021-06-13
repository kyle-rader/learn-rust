fn main() {
    println!("Hello, variables!");

    let i = 32;
    let j = 32.005;

    let k = 0x0c;

    let big = 1_304_500;

    let small = 57u8;
    let a = b'A';

    println!("i = {}", i);
    println!("j = {}", j);
    println!("k = {}", k);
    println!("big = {}", big);

    println!("small = {}", small);
    println!("a = {}", a);

    let mut foo: u8 = 254;

    println!("foo = {}", foo);
    foo += 1;
    println!("foo = {}", foo);
    // An integer overflow
    // foo += 1;
    // println!("foo = {}", foo);

    let check = '✔';
    println!("check = {}", check);
}
