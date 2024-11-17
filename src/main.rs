use std::sync::Arc;
use std::{process::Command, sync::Mutex};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::fs;

// 解决方案
struct Solution {
    score: u32,
    test: String,
}

// 测试统计
#[derive(Deserialize, Serialize)]
pub struct ExerciseStatistics {
    pub total_exercations: u32,
    pub total_succeeds: u32,
    pub total_failures: u32,
    pub total_time: u32,
}

// 统计列表
#[derive(Deserialize, Serialize)]
pub struct ExerciseCheckList {
    pub statistics: ExerciseStatistics
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 定义任务及其分数情况
    let mut test_map = HashMap::<String, Solution>::new();
    test_map.insert("solutiont1".to_owned(),  Solution{ score:  5, test: "solutiont1".to_string()});
    test_map.insert("solutiont2".to_owned(),  Solution{ score: 10, test: "solutiont2".to_string()});
    test_map.insert("solutiont3".to_owned(),  Solution{ score: 10, test: "solutiont3".to_string()});
    test_map.insert("solutiont4".to_owned(),  Solution{ score: 10, test: "solutiont4".to_string()});
    test_map.insert("solutiont5".to_owned(),  Solution{ score: 10, test: "solutiont5".to_string()});
    test_map.insert("solutiont6".to_owned(),  Solution{ score: 10, test: "solutiont6".to_string()});
    test_map.insert("solutiont7".to_owned(),  Solution{ score: 10, test: "solutiont7".to_string()});
    test_map.insert("solutiont8".to_owned(),  Solution{ score: 10, test: "solutiont8".to_string()});
    test_map.insert("solutiont9".to_owned(),  Solution{ score: 10, test: "solutiont9".to_string()});
    test_map.insert("solutiont10".to_owned(), Solution{ score: 15, test: "solutiont10".to_string()});

    // 得分统计
    let exercise_check_list = Arc::new(Mutex::new(
        ExerciseCheckList {
            statistics: ExerciseStatistics {
                total_exercations: 100,
                total_succeeds: 0,
                total_failures: 0,
                total_time: 0,
            }
        }
    ));

    // 异步执行所有测试
    let mut tasks = Vec::new();
    for (key, solution) in test_map {
        // 每个solution的结果都需要写入，需要clone避免写竞争
        let exercise_check_list_ref = Arc::clone(&exercise_check_list);
        let score = solution.score;

        // 执行topic1目录下每个solutiont(x)的测试代码
        let task = tokio::task::spawn(async move {
            run_test(&key, score, Arc::clone(&exercise_check_list_ref)).await
        });
        tasks.push(task);
    }
    for task in tasks {
        let _t = task.await?;
    }

    let serialized = serde_json::to_string_pretty(&*exercise_check_list.lock().unwrap())?;
    fs::write(".atomgit/result/check_result.json", serialized)?;

    Ok(())
}

// 异步执行测试代码
async fn run_test(key: &str, score: u32, exercise_check_list: Arc<Mutex<ExerciseCheckList>>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 构建各环境命令
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", format!("cargo test -p {}", key).as_str()])
            .output()?
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(format!("cargo test -p {}", key).as_str())
            .output()?
    };

    // 输出结果
    let stdout = String::from_utf8(output.stdout)?;
    println!("stdout: {}", stdout);
    if stdout.contains("test result: ok. 1 passed") {
        // 每通过一个加上对应得分(score)
        let mut lock = exercise_check_list.lock().unwrap();
        lock.statistics.total_succeeds += score;
    }

    Ok(())
}
