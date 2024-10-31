mod calc_logic;

#[cfg(test)]
mod tests {
    use super::calc_logic::new_birthday_probability;

    // 定义测试用例和预期结果
    const TEST_CASES: &[(u32, f64)] = &[
        (23, 0.5073),
        (30, 0.7063),
        (50, 0.9704),
        (78, 0.9999),
        (100, 1.0000),
    ];

    // 定义一个测试函数来验证每个测试用例
    #[test]
    fn test_new_birthday_probability() {
        let mut total_score = 0.0;
        for (input, expected) in TEST_CASES {
            let result = new_birthday_probability(*input);

            // 定义一个容差值
            let tolerance = 0.0001;
            if (result - expected).abs() < tolerance {
                total_score += 20.0;
            } else {
                println!(
                    "Test case n={} failed. Expected {:.4}, got {:.4}",
                    input, expected, result
                );
            }
        }
        println!("Total score: {:.2}", total_score);
        assert_eq!(100.00, total_score);
    }
}
