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

#[derive(Debug, PartialEq)]
struct Millimeters(f32);
#[derive(Debug, PartialEq)]
struct Meters(f32);

impl Add for Millimeters {
    type Output = Millimeters;
    fn add(self, rhs: Self) -> Self::Output {
        Millimeters(self.0 + rhs.0)
    }
}

impl Add for Meters {
    type Output = Meters;
    fn add(self, rhs: Self) -> Self::Output {
        Meters(self.0 + rhs.0)
    }
}

impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + rhs.0 * 1000.0)
    }
}

impl Add<Millimeters> for Meters {
    type Output = Meters;
    fn add(self, rhs: Millimeters) -> Self::Output {
        Meters(self.0 + rhs.0 / 1000.0)
    }
}

fn main() {
    println!("Hello, advanced traits!");

    let p = Point { x: 3, y: 7 };
    let q = Point { x: 1, y: -2 };

    println!("p + q = {:?}", p + q);

    println!("34mm + 2m = {:?}", Millimeters(34.0) + Meters(2.0));
    println!("1m + 255mm = {:?}", Meters(1.0) + Millimeters(255.0));

    println!("25m + 75m = {:?}", Meters(25.0) + Meters(75.0));
    println!("25mm + 75mm = {:?}", Millimeters(25.0) + Millimeters(75.0));
}

#[cfg(test)]
mod test {
    use super::{Meters, Millimeters};

    #[test]
    fn add_meters() {
        assert_eq!(Meters(3), Meters(1.0) + Meters(2.0));
    }

    #[test]
    fn add_millimeters() {
        assert_eq!(Millimeters(30.0), Millimeters(10.0) + Millimeters(20.0));
    }

    #[test]
    fn add_mm_to_m() {
        assert_eq!(Meters(5.0) + Millimeters(350.0), Meters(5.35));
    }

    #[test]
    fn add_m_to_mm() {
        assert_eq!(Millimeters(3500.0) + Meters(2.0), Millimeters(5500.0));
    }
}
