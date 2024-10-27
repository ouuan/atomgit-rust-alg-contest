mod calc_logic;

fn main() {
    // 更改这个值来测试不同的人数
    let n: u32 = 78;
    let probability = calc_logic::birthday_paradox_probability(n);
    println!(
        "在 {} 个人中，有两个人在同一天过生日的概率是 {:.2}%",
        n,
        probability * 100.0
    );
}