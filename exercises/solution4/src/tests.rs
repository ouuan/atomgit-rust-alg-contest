// src/tests.rs
mod rec_mc;
#[cfg(test)]
mod tests {
    use super::rec_mc::dp_rec_mc;
    // 定义测试用例和预期结果
    // const CASHES: [u32; 8] = [1, 2, 5, 10, 20, 30, 50, 100];
    const TEST_CASES: &[(u32, u32)] = &[
        (90, 3),
        (93, 5),
        (101, 2),
        (102, 2),
        (0, 0),
    ];
    // 定义一个测试函数来验证每个测试用例
    #[test]
    fn test_count() {
        let mut total_score = 0.0;
        for (input1, expected) in TEST_CASES {
            let result = dp_rec_mc(*input1);
            if result == *expected {
                total_score += 20.0;
            }
        }
        println!("Total score: {:.2}", total_score);
        assert_eq!(100.00, total_score);
    }
}
