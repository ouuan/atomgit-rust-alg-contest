use pinyin::han_to_wade_giles;

pub fn converter(input: &str) -> String {
    let mut result = han_to_wade_giles(input);
    if let Some((index, _)) = result.char_indices().nth(1) {
        result[..index].make_ascii_uppercase();
    } else {
        result.make_ascii_uppercase();
    }
    result
}
