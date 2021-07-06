pub fn raindrops(n: u32) -> String {
    let div_by = |d| n % d == 0;
    let mut result = String::new();

    if div_by(3) {
        result.push_str("Pling");
    }
    if div_by(5) {
        result.push_str("Plang");
    }
    if div_by(7) {
        result.push_str("Plong");
    }
    if result.len() == 0 {
        result = n.to_string();
    }
    result
}
