// src/tests.rs
mod identity_card;

#[cfg(test)]
mod tests {
    use super::identity_card::check_id_card;
    use std::time::{Instant, Duration};

    // 定义测试用例和预期结果
    // 申明：随机生成的测试身份证号，仅限测试使用，任何人不得盗用，违者按法律严惩！
    const TEST_CASES: &[(&str, &str)] = &[
        ("420111198203251029", "身份证号码正确,女,1982年03月25日,湖北省-武汉市-洪山区"),
        ("11000019900101001X", "身份证号码错误"),
        ("370725881105149", "身份证号码正确,男,1988年11月05日,山东省-潍坊市-昌乐县"),
        ("37072519881105149X", "身份证号码正确,男,1988年11月05日,山东省-潍坊市-昌乐县"),
        ("@", "身份证号码错误"),
        ("1101021990010110140", "身份证号码错误"),
        ("110102199001011014", "身份证号码正确,男,1990年01月01日,北京市-市辖区-西城区"),
        ("510303199009142328", "身份证号码正确,女,1990年09月14日,四川省-自贡市-贡井区"),
        ("320106199002071259", "身份证号码正确,男,1990年02月07日,江苏省-南京市-鼓楼区"),
        ("310104199007122348", "身份证号码正确,女,1990年07月12日,上海市-市辖区-徐汇区"),
    ];

    // 定义一个测试函数来验证每个测试用例
    #[test]
    fn test_id_card_check() {
        let mut total_score = 0.0;
        for (input, expected) in TEST_CASES {
            let start = Instant::now();
            let result = check_id_card(*input);
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
