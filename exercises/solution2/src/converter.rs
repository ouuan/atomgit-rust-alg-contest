use regex::Regex;

pub fn convert_base(num_str: &str, to_base: u32) -> String {
    assert!(to_base >= 2 && to_base <= 16, "Base must be between 2 and 16");

    let (num_str2, from_base) = parse_number_and_base(num_str).expect("invalid input format");

    // Step 1: 转为10进制
    let decimal = from_base_to_decimal(&num_str2, from_base);

    // Step 2: 从10进制转为目标进制
    from_decimal_to_base(decimal, to_base)
}

// 解析输入
fn parse_number_and_base(input: &str) -> Option<(String, u32)> {
    // 定义正则表达式
    let re = Regex::new(r"^(\d+)\((\d+)\)$").unwrap();

    // 匹配输入字符串
    if let Some(captures) = re.captures(input) {
        // 提取捕获组中的数字
        if let (Some(num_match), Some(base_match)) = (captures.get(1), captures.get(2)) {
            let num = num_match.as_str().to_string();
            if let Ok(base) = base_match.as_str().parse() {
                return Some((num, base));
            }
        }
    }

    None
}

fn from_base_to_decimal(num_str: &str, base: u32) -> u32 {
    num_str.chars().rev().enumerate().fold(0, |acc, (i, c)| {
        let digit = match c.to_digit(36) {
            Some(d) => d,
            None => panic!("Invalid character {} in base {}", c, base),
        };
        assert!(digit < base, "Digit {} is not valid for base {}", digit, base);
        acc + digit * base.pow(i as u32)
    })
}

fn from_decimal_to_base(mut num: u32, base: u32) -> String {
    if num == 0 {
        return "0".to_string();
    }

    let mut result = String::new();
    while num > 0 {
        let remainder = num % base;
        let char = std::char::from_digit(remainder, 36).unwrap();
        result.push(char);
        num /= base;
    }
    result.chars().rev().collect()
}
