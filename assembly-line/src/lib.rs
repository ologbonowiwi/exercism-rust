fn get_success_rate(speed: u8) -> f64 {
    (match speed {
        0 => 0.0,
        1..=4 => 100.0,
        5..=8 => 90.0,
        9 | 10 => 77.0,
        _ => unimplemented!("success rate inexistent for this value!")
    }) / 100.0
}

pub fn production_rate_per_hour(speed: u8) -> f64 {
    const PRODUCTION_PER_HOUR: f64 = 221.0;

    speed as f64 * PRODUCTION_PER_HOUR * (get_success_rate(speed))
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    production_rate_per_hour(speed) as u32 / 60
}
