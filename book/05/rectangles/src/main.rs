#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new_square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
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

    print_area(rect1.area());

    let rect2 = Rectangle {
        width: 80,
        height: 60,
    };

    let can_hold = if rect2.can_hold(&rect1) {
        "can hold"
    } else {
        "cannot hold"
    };
    println!("{:?} {} {:?}", rect2, can_hold, rect1);

    let rect3 = Rectangle {
        width: 5,
        height: 25,
    };

    let can_hold = if rect3.can_hold(&rect1) {
        "can hold"
    } else {
        "cannot hold"
    };
    println!("{:?} {} {:?}", rect3, can_hold, rect1);

    let square = Rectangle::new_square(15);
    println!("Square is {:?}", square);
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
