// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    println!("calculate hourly production rate at speed: {}", &speed);
    let rate = speed as f64 * 221.0;
    match speed {
        1..=4 => rate * 1.0,
        5..=8 => rate * 0.9,
        9..=10 => rate * 0.77,
        _ => 0.0
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    println!("calculate the amount of working items at speed: {}", &speed);
    production_rate_per_hour(speed) as u32 / 60
}
