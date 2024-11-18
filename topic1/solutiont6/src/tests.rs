// src/tests.rs
mod simple2traditional;

#[cfg(test)]
mod tests {
    use super::simple2traditional::converter;
    use std::time::{Instant, Duration};

    // 定义测试用例和预期结果
    const TEST_CASES: &[(&str, &str, &str)] = &[
        ("学习", "s2t", "學習"),
        ("老板", "s2t", "老闆"),
        ("四川", "s2t", "四川"),
        ("四川", "t2s", "四川"),
        ("头发", "s2t", "頭髮"),
        ("发财", "s2t", "發財"),
        ("皇后", "s2t", "皇后"),
        ("前后", "s2t", "前後"),
        ("搜刮", "s2t", "搜刮"),
        ("xx", "t2s", "xx"),
        ("", "t2s", ""),
        ("@", "t2s", "@"),
        ("0", "t2s", "0"),
        ("魏征", "s2t", "魏徵"),
        ("魏征", "t2s", "魏征"),
        ("面条", "s2t", "麵條"),
        ("小麥", "s2t", "小麥"),
        ("树干", "s2t", "樹幹"),
        ("干涉", "s2t", "干涉"),
        ("子丑寅卯", "s2t", "子丑寅卯"),
    ];

    // 定义一个测试函数来验证每个测试用例
    #[test]
    fn test_s2t_or_t2s() {
        let mut total_score = 0.0;
        for (input, tp, expected) in TEST_CASES {
            let start = Instant::now();
            let result = converter(*input, *tp);
            let duration = start.elapsed();

            // 时间超0.5s，判定不合格
            if duration <= Duration::from_millis(500) && result == *expected {
                total_score += 5.0;
            }
        }
        println!("Total score: {:.2}", total_score);
        assert_eq!(100.00, total_score);
    }
}
