const base_car_rate: f64 = 221.0;
const slow_rate: f64 = 1.0;
const average_rate: f64 = 0.9;
const fast_rate: f64 = 0.77;

fn success_rate(speed: u8) -> f64 {
    match speed {
        1..=4 => slow_rate,
        5..=8 => average_rate,
        9 | 10 => fast_rate,
        _ => 0.0,
    }
}

pub fn production_rate_per_hour(speed: u8) -> f64 {
     base_car_rate * (speed as f64) * success_rate(speed)
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
