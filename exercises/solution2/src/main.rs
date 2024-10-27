mod converter;

fn main() {
    // 更改这个值来测试不同的人数
    let num_str = "9(10)";
    let base_to: u32 = 8;
    let result = converter::convert_base(num_str, base_to);
    println!("{} -> {}({})", num_str, result, base_to);
}
