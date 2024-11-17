// src/tests.rs
mod prime_factor;

#[cfg(test)]
mod tests {
    use super::prime_factor::find_max_prime_factor;
    use std::time::{Instant, Duration};

    // 定义测试用例和预期结果
    const TEST_CASES: &[(u128, u128)] = &[
        (10000071, 370373),
        (600851475143, 6857),
        (1600851475143, 16807369),
        (76008514751430, 2163013),
        (96008514751430, 223275615701),
        (99999999951437, 5218879),
        (1199999999951437, 3945019577),
        (9999999999999951437,387792298444951),
        (97993999919999958437, 203729729563409477),
        (199999999999999951437, 9523809523809521497),
    ];

    // 定义一个测试函数来验证每个测试用例
    #[test]
    fn test_max_prime_factor() {
        let mut total_score = 0.0;
        for (input, expected) in TEST_CASES {
            let start = Instant::now();
            let result = find_max_prime_factor(*input);
            let duration = start.elapsed();

            // 时间超3s，判定不合格
            if duration <= Duration::new(3, 0) && result == *expected {
                total_score += 10.0;
            }
        }
        println!("Total score: {:.2}", total_score);
        assert_eq!(100.00, total_score);
    }
}
