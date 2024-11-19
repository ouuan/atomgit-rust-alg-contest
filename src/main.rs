use std::sync::Arc;
use std::{process::Command, sync::Mutex};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Debug)]
pub struct ExerciseList {
    pub exercises: Vec<Exercise>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Exercise {
    pub path: String,
    pub score: u32,
    pub test: String,
}

// 测试统计
#[derive(Deserialize, Serialize, Debug)]
pub struct ExerciseStatistics {
    pub total_exercations: u32,
    pub total_succeeds: u32,
    pub total_failures: u32,
    pub total_time: u32,
}

// 统计列表
#[derive(Deserialize, Serialize, Debug)]
pub struct ExerciseCheckList {
    pub statistics: ExerciseStatistics
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let toml_str = &fs::read_to_string("info.toml").unwrap();
    let exercises = toml::from_str::<ExerciseList>(toml_str).unwrap().exercises;

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

    for exercise in exercises {
        let exercise_check_list_ref = Arc::clone(&exercise_check_list);
        let score = exercise.score;
        fs::write(format!("topic1/{}/src/tests.rs", exercise.path), exercise.test).unwrap();
        let task = tokio::task::spawn(async move {
            run_test(&exercise.path, score, Arc::clone(&exercise_check_list_ref)).await
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
