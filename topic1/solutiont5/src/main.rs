mod retirement;

fn main() {
    let res = retirement::retire_time("1971-04", "原法定退休年龄55周岁女职工");
    println!("{res}");
}
