use pinyin::han_to_wade_giles;

pub fn converter(input: &str) -> String {
    let mut result = han_to_wade_giles(input);
    match result.char_indices().nth(1) {
        None => result.make_ascii_uppercase(),
        Some((index, _)) => result.split_at_mut(index).0.make_ascii_uppercase(),
    }
    result
}
