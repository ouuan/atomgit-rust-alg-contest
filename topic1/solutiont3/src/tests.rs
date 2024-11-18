// src/tests.rs
mod calc_time;

#[cfg(test)]
mod tests {
    use super::calc_time::time_info;
    use std::time::{Instant, Duration};

    // 定义测试用例和预期结果
    const TEST_CASES: &[(&str, &str)] = &[
        ("2024-11-10", "45,51,79"),
        ("2024-11-18", "47,42,70"),
        ("2024-12-31", "1,0,28"),
        ("2025-01-01", "1,364,27"),
        ("2025-12-31", "1,0,47"),
        ("2020-01-20", "4,346,4"),
        ("2021-02-13", "6,321,352"),
        ("2012-01-22", "3,344,0"),
        ("2013-02-11", "7,323,353"),
        ("2014-02-02", "5,332,381"),
    ];

    // 定义一个测试函数来验证每个测试用例
    #[test]
    fn test_calc_time() {
        let mut total_score = 0.0;
        for (input, expected) in TEST_CASES {
            let start = Instant::now();
            let result = time_info(*input);
            let duration = start.elapsed();

            // 时间超0.2s，判定不合格
            if duration <= Duration::from_millis(200) && result == *expected {
                total_score += 10.0;
            }
        }

        println!("Total score: {:.2}", total_score);
        assert_eq!(100.00, total_score);
    }
}
