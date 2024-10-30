
// src/tests.rs
mod converter;
#[cfg(test)]
mod tests {
    use super::converter::convert_base;

    // 定义测试用例和预期结果
    const TEST_CASES: &[(&str, u32, &str)] = &[
        ("10(2)", 10, "2"),
        ("9(10)", 8, "11"),
        ("1111(2)", 15, "10"),
        ("10(7)", 9, "7"),
        ("12(10)", 16, "c"),
    ];

    // 定义一个测试函数来验证每个测试用例
    #[test]
    fn test_converter() {
        let mut total_score = 0.0;

        for (input1, input2, expected) in TEST_CASES {
            let result = convert_base(*input1, *input2);

            if result == *expected {
                total_score += 20.0;
            }
        }
        println!("Total score: {:.2}", total_score);
        assert_eq!(100.00, total_score);
    }
}
