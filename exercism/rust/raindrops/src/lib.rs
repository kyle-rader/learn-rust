pub fn raindrops(n: u32) -> String {
    let div_by = |d: u32| n % d == 0;
    let conds = [(3, "Pling"), (5, "Plang"), (7, "Plong")];
    let result: String = conds
        .iter()
        .filter_map(|item| if div_by(item.0) { Some(item.1) } else { None })
        .collect();

    match result.len() {
        0 => n.to_string(),
        _ => result
    }
}
