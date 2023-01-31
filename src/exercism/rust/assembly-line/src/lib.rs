pub fn production_rate_per_hour(speed: u8) -> f64 {
    let base_production = 221_f64;
    let success_rate: f64 = match speed {
        0 => 0.0,
        1..=4 => 1.0,
        5..=8 => 0.9,
        9..=10 => 0.77,
        _ => unreachable!(),
    };

    base_production * speed as f64 * success_rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let rate_per_hour = production_rate_per_hour(speed);
    (rate_per_hour / 60.0).floor() as u32
}
