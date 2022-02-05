use std::slice;

unsafe fn dangerous() {}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    println!("Hello, safe unsafe Rust!");

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);
    println!("a: {:?}", &a);
    println!("b: {:?}", &b);

    let (c, d) = split_at_mut(a, 1);
    println!("c: {:?}", &c);
    println!("d: {:?}", &d);

    unsafe {
        dangerous();
    }
}
