// src/tests.rs
mod district;

#[cfg(test)]
mod tests {
    use super::district::count_provinces;
    use std::time::{Instant, Duration};

    // 定义测试用例和预期结果
    const TEST_CASE: &str = "3,3,2,2,1";

    // 定义一个测试函数来验证每个测试用例
    #[test]
    fn test_count_provinces() {
        let start = Instant::now();
        let result = count_provinces();
        let duration = start.elapsed();

        // 时间超1s，判定不合格
        let mut total_score = 0.0;
        if duration <= Duration::new(1, 0) && result == TEST_CASE {
            total_score += 100.0;
        }

        println!("Total score: {:.2}", total_score);
        assert_eq!(100.00, total_score);
    }
}
