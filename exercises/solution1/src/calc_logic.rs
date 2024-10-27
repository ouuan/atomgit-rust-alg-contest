// 生日悖论
pub fn birthday_paradox_probability(n: u32) -> f64 {
    if n < 2 {
        return 0.0;
    }

    let days_in_year = 365;
    let mut probability_no_same_birthday = 1.0;

    for i in 0..n {
        probability_no_same_birthday *= (days_in_year as f64 - i as f64) / days_in_year as f64;
    }

    1.0 - probability_no_same_birthday
}
