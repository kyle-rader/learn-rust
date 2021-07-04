fn main() {
    let w = 30;
    let h = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(w, h)
    );

    let dims = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(dims)
    );
}

fn area(w: u32, h: u32) -> u32 {
    w * h
}

fn area_tuple(dims: (u32, u32)) -> u32 {
    dims.0 * dims.1
}