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

#[derive(Debug)]
struct Millimeters(u32);
#[derive(Debug)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + rhs.0 * 1000)
    }
}

impl Add<Millimeters> for Meters {
    type Output = Millimeters;

    fn add(self, rhs: Millimeters) -> Self::Output {
        Millimeters(self.0 * 1000 + rhs.0)
    }
}

fn main() {
    println!("Hello, advanced traits!");

    let p = Point { x: 3, y: 7 };
    let q = Point { x: 1, y: -2 };

    println!("p + q = {:?}", p + q);

    println!("34mm + 2m = {:?}", Millimeters(34) + Meters(2));
    println!("1m + 255mm = {:?}", Meters(1) + Millimeters(255));
}
