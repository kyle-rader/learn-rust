pub fn is_armstrong_number(num: u32) -> bool {
    let num_but_stringly = format!("{num}");
    let power = num_but_stringly.len() as u32;

    let result: u32 = num_but_stringly
        .chars()
        .map(|c| (c as u8) - b'0')
        .map(|n| n as u32)
        .map(|n| n.pow(power))
        .sum();

    result == num
}
