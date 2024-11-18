// src/tests.rs
mod retirement;

#[cfg(test)]
mod tests {
    use super::retirement::retire_time;
    use std::time::{Instant, Duration};

    // 定义测试用例和预期结果
    const TEST_CASES: &[(&str, &str, &str)] = &[
        ("1971-04", "原法定退休年龄55周岁女职工", "2026-08,55.33,4"),
        ("1995-12", "原法定退休年龄50周岁女职工", "2050-12,55,60"),
        ("1995-12", "男职工", "2058-12,63,36"),
        ("2000-12", "原法定退休年龄55周岁女职工", "2058-12,58,36"),
        ("2000-12", "男职工", "2063-12,63,36"),
        ("1965-12", "男职工", "2026-03,60.25,3"),
        ("1963-12", "男职工", "2023-12,60,0"),
        ("1963-04", "原法定退休年龄55周岁女职工", "2018-04,55,0"),
        ("1964-02", "男职工", "2024-02,60,0"),
        ("1965-01", "男职工", "2025-02,60.08,1"),
    ];

    // 定义一个测试函数来验证每个测试用例
    #[test]
    fn test_retirement_time() {
        let mut total_score = 0.0;
        for (time, tp, expected) in TEST_CASES {
            let start = Instant::now();
            let result = retire_time(*time, *tp);
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
