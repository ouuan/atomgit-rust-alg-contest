use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufWriter, Write};

const CHARACTER_DICT: &str = include_str!("../pinyin-data/pinyin.txt");
const PHRASE_DICT: &str = include_str!("../phrase-pinyin-data/pinyin.txt");

const TONES: &[(&str, &str)] = &[
    ("ā", "a"),
    ("á", "a"),
    ("ǎ", "a"),
    ("à", "a"),
    ("ē", "e"),
    ("é", "e"),
    ("ě", "e"),
    ("è", "e"),
    ("ō", "o"),
    ("ó", "o"),
    ("ǒ", "o"),
    ("ò", "o"),
    ("ī", "i"),
    ("í", "i"),
    ("ǐ", "i"),
    ("ì", "i"),
    ("ū", "u"),
    ("ú", "u"),
    ("ǔ", "u"),
    ("ù", "u"),
    ("ü", "v"),
    ("ǖ", "v"),
    ("ǘ", "v"),
    ("ǚ", "v"),
    ("ǜ", "v"),
    ("ń", "n"),
    ("ň", "n"),
    ("ǹ", "n"),
    ("m̄", "m"),
    ("ḿ", "m"),
    ("m̀", "m"),
    ("ê̄", "ê"),
    ("ế", "ê"),
    ("ê̌", "ê"),
    ("ề", "ê"),
];

fn remove_tone(pinyin: &str) -> String {
    let mut pinyin = pinyin.trim().to_string();
    for (from, to) in TONES {
        pinyin = pinyin.replace(from, to);
    }
    pinyin
}

fn parse_character_rule(line: &str) -> Option<(String, String)> {
    let mut parts = line.split('#');
    let not_comment = parts.next()?;
    let mut parts = not_comment.split_whitespace();
    let code = parts.next()?;
    let code = code.split_at(2).1;
    let code = code.split_at(code.len() - 1).0;
    let code = u32::from_str_radix(code, 16).ok()?;
    let char = std::char::from_u32(code)?;
    let pinyin = parts.next()?;
    let pinyin = pinyin.split(',').next()?;
    let pinyin = remove_tone(pinyin);
    Some((char.into(), pinyin))
}

fn parse_phrase_rule(line: &str) -> Option<(String, String)> {
    let mut parts = line.split('#');
    let not_comment = parts.next()?;
    let mut parts = not_comment.split(':');
    let phrase = parts.next()?;
    let pinyin = parts.next()?;
    let pinyin = remove_tone(pinyin);
    Some((phrase.into(), pinyin))
}

fn main() {
    let rules = CHARACTER_DICT
        .lines()
        .filter_map(parse_character_rule)
        .chain(PHRASE_DICT.lines().filter_map(parse_phrase_rule));

    let mut map = HashMap::new();
    let mut homophone = HashSet::new();

    for (phrase, pinyin) in rules.clone() {
        assert_eq!(phrase.chars().count(), pinyin.split_whitespace().count());
        for (c, p) in phrase.chars().zip(pinyin.split_whitespace()) {
            match map.entry(c) {
                Entry::Vacant(entry) => {
                    entry.insert(p.to_string());
                }
                Entry::Occupied(entry) => {
                    if entry.get() != p {
                        homophone.insert(c);
                    }
                }
            }
        }
    }

    let file = File::create("data/dict.txt").unwrap();
    let mut writer = BufWriter::new(file);
    for (phrase, pinyin) in rules {
        if phrase.chars().count() > 1 && phrase.chars().all(|c| !homophone.contains(&c)) {
            continue;
        }
        writer.write_all(phrase.as_bytes()).unwrap();
        writer.write_all(b":").unwrap();
        writer.write_all(pinyin.as_bytes()).unwrap();
        writer.write_all(b"\n").unwrap();
    }
    writer.flush().unwrap();
}
