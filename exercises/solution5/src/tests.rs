
// src/tests.rs
mod fibnacci;
#[cfg(test)]
mod tests {
    use super::fibnacci::odd_fibnacci_sum;
    // 定义测试用例和预期结果
    const TEST_CASES: &[(u32, u32)] = &[
        (20, 23),
        (22, 44),
        (30, 44),
        (40, 44),
        (56, 99),
    ];
    // 定义一个测试函数来验证每个测试用例
    #[test]
    fn test_count() {
        let mut total_score = 0.0;
        for (input1, expected) in TEST_CASES {
            let result = odd_fibnacci_sum(*input1);
            if result == *expected {
                total_score += 20.0;
            }
        }
        println!("Total score: {:.2}", total_score);
        assert_eq!(100.00, total_score);
    }
}
