// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let rate = (speed as f64) * 221.0;
    match speed {
        0..=4 => rate,
        5..=8 => rate * 0.9,
        9..=10 => rate * 0.77,
        _ => panic!("Unknown speed!"),
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
