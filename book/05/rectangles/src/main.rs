#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn print_area(a: u32) {
    println!("The area of the rectangle is {}.", a);
}

fn main() {
    let w = 30;
    let h = 50;

    print_area(area(w, h));

    let dims = (30, 50);
    print_area(area_tuple(dims));

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    print_area(area_struct(&rect1));
    println!("rect1 is {:#?}", rect1);
}

fn area(w: u32, h: u32) -> u32 {
    w * h
}

fn area_tuple(dims: (u32, u32)) -> u32 {
    dims.0 * dims.1
}

fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}