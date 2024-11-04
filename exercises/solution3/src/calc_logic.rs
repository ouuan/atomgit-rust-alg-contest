pub fn new_birthday_probability(n: u32) -> f64 {
    if n >= 366 {
        1.0
    } else {
        1.0 - (0..n).map(|i| (365.0 - i as f64) / 365.0).product::<f64>()
    }
}
