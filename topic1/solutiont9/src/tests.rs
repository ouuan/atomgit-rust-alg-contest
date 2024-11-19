// src/tests.rs
mod spiral_prime;

#[cfg(test)]
mod tests {
    use super::spiral_prime::min_edge_prime_num;
    use std::time::{Instant, Duration};

    // 定义测试用例和预期结果
    const TEST_CASES: &[(u32, &str)] = &[
        (60, "5,5"),
        (55, "9,9"),
        (50, "11,10"),
        (40, "31,23"),
        (30, "49,28"),
        (20, "309,123"),
        (10, "26241,5248"),
        (9,  "74373,13387"),
        (8,  "238733,38197"),
        (7,  "1213001,169820"),
    ];

    // 定义一个测试函数来验证每个测试用例
    #[test]
    fn test_prime_percent() {
        let mut total_score = 0.0;
        for (input, expected) in TEST_CASES {
            let start = Instant::now();
            let result = min_edge_prime_num(*input);
            let duration = start.elapsed();

            // 时间超6s，判定不合格
            if duration <= Duration::new(6, 0) && result == *expected {
                total_score += 10.0;
            }
        }
        println!("Total score: {:.2}", total_score);
        assert_eq!(100.00, total_score);
    }
}
