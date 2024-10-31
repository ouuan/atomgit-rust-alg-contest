// src/tests.rs
mod count_distinct;

#[cfg(test)]
mod tests {
    use super::count_distinct::new_count_distinct;
    // 定义测试用例和预期结果
    const TEST_CASES: &[(&str, usize)] = &[
        ("a,b,c,a,e,cd", 5),
        ("a,b,a,a,e,cd", 4),
        ("j,a,c,d,e,z", 6),
        ("a,b,c,好,好,爱", 5),
        ("a,b,c,0,e,cd", 6),
    ];
    // 定义一个测试函数来验证每个测试用例
    #[test]
    fn test_count() {
        let mut total_score = 0.0;
        for (input1, expected) in TEST_CASES {
            let result = new_count_distinct(*input1);
            if result == *expected {
                total_score += 20.0;
            }
        }
        println!("Total score: {:.2}", total_score);
        assert_eq!(100.00, total_score);
    }
}
