pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // parse input
    let (num_str, r) = num_str.split_once('(').unwrap();
    let (from_base, _) = r.split_once(')').unwrap();
    let from_base = from_base.parse::<u32>().unwrap();

    // convert input string to integer
    let mut num = 0;
    for c in num_str.chars() {
        num = num * from_base + c.to_digit(from_base).unwrap();
    }

    // convert integer to output string
    let mut result = Vec::new();
    while num > 0 {
        result.push(std::char::from_digit(num % to_base, to_base).unwrap());
        num /= to_base;
    }
    return result.iter().rev().collect();
}
