use std::io;

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

    let symbols: (char, char, char) = ('✅', '❌', '⚠');

    println!(
        "We have symbols (from a tuple) {}, {}, and {}",
        symbols.0, symbols.1, symbols.2
    );

    let a = [1, 2, 3, 4, 5];
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect(format!("{} Failed to read line!", symbols.1).as_str());

    let index :usize = index
        .trim()
        .parse()
        .expect(format!("{} Failed to parse index!", symbols.1).as_str());

    println!("Value of array at index {} is {}", index, a[index]);
}
