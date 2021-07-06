pub fn raindrops(n: u32) -> String {
    let div_by = |d: u32| n % d == 0;

    let result: String = [(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .iter()
        .filter(|item| div_by(item.0))
        .map(|item| item.1)
        .collect();

    match result.len() {
        0 => n.to_string(),
        _ => result
    }
}
