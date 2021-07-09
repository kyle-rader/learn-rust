use rand::Rng;

/// Add one to a given value
/// # Examples
/// ```
/// use add_one::one_more;
/// assert_eq!(one_more(2), 3);
/// ```
pub fn one_more(x: i32) -> i32 {
    x + 1
}

pub fn some_more(x: i32) -> i32 {
    let mut r = rand::thread_rng();
    let j = r.gen_range(1..100);
    x + j
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(one_more(5), 6);
        assert_eq!(one_more(6), 7);
        assert_eq!(one_more(one_more(6)), 8);
    }
}
