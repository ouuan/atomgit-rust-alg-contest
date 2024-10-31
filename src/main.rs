use std::sync::Arc;
use std::{process::Command, sync::Mutex};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::fs;

#[tokio::main]
async fn main() {
    let solution1 = format!("
// src/tests.rs
mod count_distinct;

#[cfg(test)]
mod tests {{
    use super::count_distinct::new_count_distinct;
    // 定义测试用例和预期结果
    const TEST_CASES: &[(&str, usize)] = &[
        (\"a,b,c,a,e,cd\", 5),
        (\"a,b,a,a,e,cd\", 4),
        (\"j,a,c,d,e,z\", 6),
        (\"a,b,c,好,好,爱\", 5),
        (\"a,b,c,0,e,cd\", 6),
    ];

    // 定义一个测试函数来验证每个测试用例
    #[test]
    fn test_count() {{
        let mut total_score = 0.0;
        for (input1, expected) in TEST_CASES {{
            let result = new_count_distinct(*input1);
            if result == *expected {{
                total_score += 20.0;
            }}
        }}
        println!(\"Total score: {{:.2}}\", total_score);
        assert_eq!(100.00, total_score);
    }}
}}
    ");
    let solution2 = format!("
// src/tests.rs
mod converter;

#[cfg(test)]
mod tests {{
    use super::converter::convert_base;

    // 定义测试用例和预期结果
    const TEST_CASES: &[(&str, u32, &str)] = &[
        (\"10(2)\", 10, \"2\"),
        (\"9(10)\", 8, \"11\"),
        (\"1111(2)\", 15, \"10\"),
        (\"10(7)\", 9, \"7\"),
        (\"12(10)\", 16, \"c\"),
    ];

    // 定义一个测试函数来验证每个测试用例
    #[test]
    fn test_converter() {{
        let mut total_score = 0.0;

        for (input1, input2, expected) in TEST_CASES {{
            let result = convert_base(*input1, *input2);
            if result == *expected {{
                total_score += 20.0;
            }}
        }}
        println!(\"Total score: {{:.2}}\", total_score);
        assert_eq!(100.00, total_score);
    }}
}}
    ");
    let solution3 = format!("
mod calc_logic;

#[cfg(test)]
mod tests {{
    use super::calc_logic::new_birthday_probability;
    // 定义测试用例和预期结果
    const TEST_CASES: &[(u32, f64)] = &[
        (23, 0.5073),
        (30, 0.7063),
        (50, 0.9704),
        (78, 0.9999),
        (100, 1.0000),
    ];

    // 定义一个测试函数来验证每个测试用例
    #[test]
    fn test_new_birthday_probability() {{
        let mut total_score = 0.0;
        for (input, expected) in TEST_CASES {{
            let result = new_birthday_probability(*input);

            // 定义一个容差值
            let tolerance = 0.0001;
            if (result - expected).abs() < tolerance {{
                total_score += 20.0;
            }} else {{
                println!(
                    \"Test case n={{}} failed. Expected {{:.4}}, got {{:.4}}\",
                    input, expected, result
                );
            }}
        }}
        println!(\"Total score: {{:.2}}\", total_score);
        assert_eq!(100.00, total_score);
    }}
}}
    ");
    let solution4 = format!("
// src/tests.rs
mod rec_mc;

#[cfg(test)]
mod tests {{
    use super::rec_mc::dp_rec_mc;
    // 定义测试用例和预期结果
    const TEST_CASES: &[(u32, u32)] = &[
        (90, 3),
        (93, 5),
        (101, 2),
        (102, 2),
        (0, 0),
    ];

    // 定义一个测试函数来验证每个测试用例
    #[test]
    fn test_count() {{
        let mut total_score = 0.0;
        for (input1, expected) in TEST_CASES {{
            let result = dp_rec_mc(*input1);
            if result == *expected {{
                total_score += 20.0;
            }}
        }}
        println!(\"Total score: {{:.2}}\", total_score);
        assert_eq!(100.00, total_score);
    }}
}}
    ");
    let solution5 = format!("
// src/tests.rs
mod fibnacci;

#[cfg(test)]
mod tests {{
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
    fn test_count() {{
        let mut total_score = 0.0;
        for (input1, expected) in TEST_CASES {{
            let result = odd_fibnacci_sum(*input1);
            if result == *expected {{
                total_score += 20.0;
            }}
        }}
        println!(\"Total score: {{:.2}}\", total_score);
        assert_eq!(100.00, total_score);
    }}
}}
    ");

    let mut test_map = HashMap::<String, Solution>::new();
    test_map.insert("solution1".to_owned(), Solution{ score: 20 ,test: solution1});
    test_map.insert("solution2".to_owned(), Solution{ score: 20 ,test: solution2});
    test_map.insert("solution3".to_owned(), Solution{ score: 20 ,test: solution3});
    test_map.insert("solution4".to_owned(), Solution{ score: 20 ,test: solution4});
    test_map.insert("solution5".to_owned(), Solution{ score: 20 ,test: solution5});

    let exercise_check_list =  Arc::new(Mutex::new(
        ExerciseCheckList {
            statistics: ExerciseStatistics {
                total_exercations: 100,
                total_succeeds: 0,
                total_failures: 0,
                total_time: 0,
            }
        }
    ));

    let mut tasks = Vec::new();
    for (key, solution) in test_map {
        // 每个solution的结果都需要写入，需要clone下
        let exercise_check_list_ref = Arc::clone(&exercise_check_list);

        let score = solution.score;
        let value = solution.test.clone();
        let t = tokio::task::spawn(async move {
            // 写测试代码
            let path = format!("exercises/{}/src/tests.rs", key);
            fs::write(path, value).expect("Failed to write test file");

            // 执行测试代码
            let output = if cfg!(target_os = "windows") {
                Command::new("cmd")
                    .args(["/C", format!("cargo test -p {}", key).as_str()])
                    .output()
                    .expect("failed to execute process")
            } else {
                Command::new("sh")
                    .arg("-c")
                    .arg(format!("cargo test -p {}", key).as_str())
                    .output()
                    .expect("failed to execute process")
            };

            // 每通过一个加20分(score)
            let stdout = String::from_utf8(output.stdout).unwrap();
            println!("stdout: {}", stdout);
            if stdout.contains("test result: ok. 1 passed") {
                exercise_check_list_ref.lock().unwrap().statistics.total_succeeds += score;
            }
        });
        tasks.push(t);
    }
    // 异步执行所有测试
    for task in tasks { task.await.unwrap(); }

    let serialized = serde_json::to_string_pretty(&*exercise_check_list.lock().unwrap()).unwrap();
    fs::write(".atomgit/result/check_result.json", serialized).unwrap();
}

struct Solution {
    score: u32,
    test: String,
}


#[derive(Deserialize, Serialize)]
pub struct ExerciseStatistics {
    pub total_exercations: u32,
    pub total_succeeds: u32,
    pub total_failures: u32,
    pub total_time: u32,
}

#[derive(Deserialize, Serialize)]
pub struct ExerciseCheckList {
    pub statistics: ExerciseStatistics
}
