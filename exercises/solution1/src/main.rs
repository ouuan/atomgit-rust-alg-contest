mod count_distinct;

fn main() {
    let input_str: &str = "a,b,cd,b,e,e,d,a";
    let count = count_distinct::new_count_distinct(input_str);
    println!("count: {count}");
}
