use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    println!("Hello, advanced traits!");

    let p = Point { x: 3, y: 7 };
    let q = Point { x: 1, y: -2 };

    println!("p + q = {:?}", p + q);
}
