// src/tests.rs
mod zuc_encryption;

#[cfg(test)]
mod tests {
    use super::zuc_encryption::encryption;
    use std::time::{Instant, Duration};

    // 定义测试用例和预期结果
    const TEST_CASES: &[(&str, &str)] = &[
        ("特朗普", "/QZb7S0JWp/IYuwB"),
        ("吉普车", "/x9r7SgwVLvAYuwB"),
        ("中国人", "/jdP7iojWLzcYuwB"),
        ("RustRover", "SPqRf+PxymMUYuwB"),
        ("@####", "WqzBKJKdvwU="),
        ("0", "KozhCA=="),
        ("深心托豪素,怀抱观古今", "/DhT7g4dWo/+iV6oScyAPMtykWeYW34bCcaYzZudbAU="),
        ("境外势力", "/y1h7hUIWYzZhGWZqnwkFA=="),
        ("信息化数字化智能化", "/jBD7TAxWYrwh3qyS9W39aFk9xioAhUBbq+BaA=="),
        ("Talk is cheap, show me the code", "Tu6OYJH3zyYFCYpj3lQAY0WdZqF/j7bI40Y3ChBCgwU=")
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
