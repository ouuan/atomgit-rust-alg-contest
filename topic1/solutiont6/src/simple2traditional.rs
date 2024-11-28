use chinese_convert::*;

pub fn converter(input: &str, tp: &str) -> String {
    match tp {
        "s2t" => traditional_to_tw(&simplified_to_traditional(input)),
        "t2s" => traditional_to_simplified(input),
        _ => panic!("Invalid type"),
    }
}
