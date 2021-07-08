#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn add_2(a: i32) -> i32 {
    a + 2
}

fn double(s: &str) -> String {
    format!("{}{}", s, s)
}

fn replicate(s: &str, n: i32) -> String {
    let mut result = String::new();
    for _ in 0..n {
        result.push_str(s);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 5, height: 5 };
        let smaller = Rectangle { width: 3, height: 4 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 5, height: 5 };
        let smaller = Rectangle { width: 3, height: 4 };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn matching_cannot_hold_matching() {
        let same = Rectangle { width: 5, height: 5};

        assert!(!same.can_hold(&same));
    }

    #[test]
    fn add_2_works() {
        assert_eq!(add_2(4), 6);
    }

    #[test]
    fn double_works() -> Result<(), String> {
        let result = double("abc");
        let expected = "abcabc";
        if result.contains(expected) {
            Ok(())
        } else {
            Err(format!("{} did not contain {}", result, expected))
        }
    }

    #[test]
    fn replicate_works() {
        let result = replicate("a", 3);
        assert_eq!(result, "aaa".to_string());
    }

    #[test]
    #[should_panic(expected = "Testing failure states is important")]
    fn asserting_panic() {
        // panic!("Oops");
        panic!("Testing failure states is important too!");
    }
}
