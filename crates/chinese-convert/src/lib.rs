use max_match_segmentation::{StringFmm as Converter, StringSegmentation};
use std::sync::LazyLock;

const ST: &[&str] = &[
    include_str!("../data/STCharacters.txt"),
    include_str!("../data/STPhrases.txt"),
    include_str!("../data/STFix.txt"),
];
const TS: &[&str] = &[
    include_str!("../data/TSCharacters.txt"),
    include_str!("../data/TSPhrases.txt"),
];
const TW: &[&str] = &[
    include_str!("../data/TWVariants.txt"),
    include_str!("../data/TWPhrasesIT.txt"),
    include_str!("../data/TWPhrasesName.txt"),
    include_str!("../data/TWPhrasesOther.txt"),
];

fn parse_dict(line: &str) -> (&str, &str) {
    let mut parts = line.split_whitespace();
    let from = parts.next().unwrap();
    let to = parts.next().unwrap();
    (from, to)
}

fn load_dicts(dicts: &[&'static str]) -> Converter<&'static str> {
    let mut converter = Converter::new();
    for line in dicts.iter().flat_map(|s| s.lines()) {
        let (from, to) = parse_dict(line);
        converter.add_phrase(from, to);
    }
    converter
}

static ST_CONVERTER: LazyLock<Converter<&str>> = LazyLock::new(|| load_dicts(ST));

static TS_CONVERTER: LazyLock<Converter<&str>> = LazyLock::new(|| load_dicts(TS));

static TW_CONVERTER: LazyLock<Converter<&str>> = LazyLock::new(|| load_dicts(TW));

pub fn simplified_to_traditional(simplified: &str) -> String {
    ST_CONVERTER.convert(simplified, "")
}

pub fn traditional_to_simplified(traditional: &str) -> String {
    TS_CONVERTER.convert(traditional, "")
}

pub fn traditional_to_tw(traditional: &str) -> String {
    TW_CONVERTER.convert(traditional, "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s2t_single() {
        const TESTS: &[(&str, &str)] = &[
            ("㓦划", "㓦劃"),
            ("一伙人", "一夥人"),
            ("饥", "飢"),
            ("面条", "麪條"),
        ];

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

    #[test]
    fn t2tw() {
        const TESTS: &[(&str, &str)] = &[
            ("麪條", "麵條"),
            ("U盤", "隨身碟"),
            ("海內存知己", "海內存知己"),
        ];
        for (t, tw) in TESTS {
            assert_eq!(&traditional_to_tw(t), tw);
        }
    }
}
