use max_match_segmentation::{Segment, StringBimm as Converter, StringSegmentation};
use std::collections::HashMap;
use std::sync::LazyLock;

mod constants;
use constants::*;

static WADE_MAP: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    for &(from, to) in WADE_LIST {
        if from != to {
            map.insert(from, to);
        }
    }
    map
});

static PINYIN_CONVERTER: LazyLock<Converter<&str>> = LazyLock::new(|| {
    let mut converter = Converter::new();
    for line in DICT_STR.lines() {
        let (phrase, pinyin) = line.split_once(':').unwrap();
        converter.add_phrase(phrase, pinyin);
    }
    converter
});

pub fn han_to_pinyin(han: &str) -> String {
    let mut result = String::new();
    for seg in PINYIN_CONVERTER.segmentation(han) {
        if let Segment::Match(pinyin, _) = seg {
            if !result.is_empty() {
                result += " ";
            }
            result += pinyin;
        }
    }
    result
}

pub fn pinyin_to_wade_giles(pinyin: &str) -> String {
    pinyin
        .split_whitespace()
        .map(|p| *WADE_MAP.get(p).unwrap_or(&p))
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn han_to_wade_giles(han: &str) -> String {
    pinyin_to_wade_giles(&han_to_pinyin(han))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_han_to_pinyin() {
        assert_eq!(han_to_pinyin("测试 test"), "ce shi");
        assert_eq!(han_to_pinyin("欢乐的乐曲"), "huan le de yue qu");
    }

    #[test]
    fn test_han_to_wade_giles() {
        assert_eq!(han_to_wade_giles("清华大学"), "ch'ing hua ta hsüeh");
        assert_eq!(han_to_wade_giles("北京大学"), "pei ching ta hsüeh");
    }
}
