// src/tests.rs
mod wade_giles_romanization;

#[cfg(test)]
mod tests {
    use super::wade_giles_romanization::converter;
    use std::time::{Instant, Duration};

    // 定义测试用例和预期结果
    const TEST_CASES: &[(&str, &str)] = &[
        ("中国", "Chung kuo"),
        ("@", ""),
        ("诸葛亮", "Chu ko liang"),
        ("孙中山", "Sun chung shan"),
        ("台湾", "T'ai wan"),
        ("香港", "Hsiang kang"),
        ("澳門", "Ao men"),
        ("川普", "Ch'uan p'u"),
        ("四川", "Ssu ch'uan"),
        ("廣州", "Kuang chou"),
    ];

    // 定义一个测试函数来验证每个测试用例
    #[test]
    fn test_wadegiles() {
        let mut total_score = 0.0;
        for (input, expected) in TEST_CASES {
            let start = Instant::now();
            let result = converter(*input);
            let duration = start.elapsed();

            // 时间超0.5s，判定不合格
            if duration <= Duration::from_millis(500) && result == *expected {
                total_score += 10.0;
            }
        }
        println!("Total score: {:.2}", total_score);
        assert_eq!(100.00, total_score);
    }
}
