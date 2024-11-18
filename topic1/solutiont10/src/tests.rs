// src/tests.rs
mod zuc_encryption;

#[cfg(test)]
mod tests {
    use super::zuc_encryption::encryption;
    use std::time::{Instant, Duration};

    // 定义测试用例和预期结果
    const TEST_CASES: &[(&str, &str)] = &[
        ("特朗普", "/QZb7S0JWp8="),
        ("吉普车", "/x9r7SgwVLs="),
        ("中国人", "/jdP7iojWLw="),
        ("RustRover", "SPqRf+PxymM="),
        ("@####", "WqzBKA=="),
        ("0", ""),
        ("深心托豪素,怀抱观古今", "/DhT7g4dWo/QW9MZ1Mz5asg3lTdoQFt4ga9FMQ=="),
        ("境外势力", "/y1h7hUIWYz3Vugo"),
        ("信息化数字化智能化", "/jBD7TAxWYreVfcD1tXOo6Ih80hYGTBi"),
        ("Talk is cheap, show me the code", "Tu6OYJH3zyYr2wfSQ1R5NUbYYvGPlJOray/q9g=="),
    ];


    // 定义一个测试函数来验证每个测试用例
    #[test]
    fn test_zuc_encryption() {
        let mut total_score = 0.0;
        for (input, expected) in TEST_CASES {
            let start = Instant::now();
            let result = encryption((*input).to_string());
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
