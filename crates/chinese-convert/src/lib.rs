use max_match_segmentation::{StringFmm as Converter, StringSegmentation};
use std::sync::LazyLock;

const ST_CHAR: &str = include_str!("../data/STCharacters.txt");
const ST_PHRASE: &str = include_str!("../data/STPhrases.txt");
const TS_CHAR: &str = include_str!("../data/TSCharacters.txt");
const TS_PHRASE: &str = include_str!("../data/TSPhrases.txt");

const ST_SP: &[(&str, &str)] = &[("子丑寅卯", "子丑寅卯"), ("面条", "麵條")];
const TS_SP: &[(&str, &str)] = &[];

fn parse_dict(line: &str) -> (&str, &str) {
    let mut parts = line.split_whitespace();
    let from = parts.next().unwrap();
    let to = parts.next().unwrap();
    (from, to)
}

static ST_CONVERTER: LazyLock<Converter<&str>> = LazyLock::new(|| {
    let mut converter = Converter::new();
    for line in ST_CHAR.lines().chain(ST_PHRASE.lines()) {
        let (from, to) = parse_dict(line);
        converter.add_phrase(from, to);
    }
    for (s, t) in ST_SP {
        converter.add_phrase(s, t);
    }
    converter
});

static TS_CONVERTER: LazyLock<Converter<&str>> = LazyLock::new(|| {
    let mut converter = Converter::new();
    for line in TS_CHAR.lines().chain(TS_PHRASE.lines()) {
        let (from, to) = parse_dict(line);
        converter.add_phrase(from, to);
    }
    for (t, s) in TS_SP {
        converter.add_phrase(t, s);
    }
    converter
});

pub fn simplified_to_traditional(simplified: &str) -> String {
    ST_CONVERTER.convert(simplified, "")
}

pub fn traditional_to_simplified(traditional: &str) -> String {
    TS_CONVERTER.convert(traditional, "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s2t_single() {
        const TESTS: &[(&str, &str)] = &[("㓦划", "㓦劃"), ("一伙人", "一夥人"), ("饥", "飢")];

        for (s, t) in TESTS {
            assert_eq!(&simplified_to_traditional(s), t);
        }
    }

    #[test]
    fn s2t_segmentation() {
        const TESTS: &[(&str, &str)] = &[("可怜白发生", "可憐白髮生")];

        for (s, t) in TESTS {
            assert_eq!(&simplified_to_traditional(s), t);
        }
    }

    #[test]
    fn t2s_single() {
        const TESTS: &[(&str, &str)] = &[("一目瞭然", "一目了然"), ("龍鍾", "龙钟"), ("㑮", "𫝈")];
        for (t, s) in TESTS {
            assert_eq!(&traditional_to_simplified(t), s);
        }
    }

    #[test]
    fn t2s_segmentation() {
        const TESTS: &[(&str, &str)] = &[
            ("電覆盆子", "电复盆子"),
            ("大目乾連冥間救母變文錦覆阱", "大目乾连冥间救母变文锦覆阱"),
        ];
        for (t, s) in TESTS {
            assert_eq!(&traditional_to_simplified(t), s);
        }
    }
}
