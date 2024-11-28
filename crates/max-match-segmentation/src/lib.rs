use std::collections::HashMap;
use std::fmt::{Display, Write};
use std::hash::Hash;

/// General forward maximum matching segmentation with associated attributes.
pub struct Fmm<T, V> {
    root: HashMap<T, Node<T, V>>,
}

struct Node<T, V> {
    attr: Option<V>,
    children: HashMap<T, Self>,
}

/// Segentation result
#[derive(Debug, PartialEq, Eq)]
pub enum Segment<T, V> {
    /// associated attribute and the range [l, r) in the original sentence
    Match(V, (usize, usize)),
    /// unmatched token and the position in the original sentence
    Unmatched(T, usize),
}

impl<T, V> Fmm<T, V>
where
    T: Eq + Hash,
{
    /// Adds a phrase with attribute to the dictionary.
    ///
    /// # Panics
    ///
    /// Panics if the phrase is empty.
    pub fn add_phrase<I>(&mut self, phrase: I, attr: V)
    where
        I: IntoIterator<Item = T>,
    {
        let mut phrase = phrase.into_iter();
        let mut node = self
            .root
            .entry(phrase.next().expect("empty phrase"))
            .or_default();
        for c in phrase {
            node = node.children.entry(c).or_default();
        }
        node.attr = Some(attr);
    }

    /// Forward maximum matching segmentation for the input sentence.
    pub fn segmentation<I>(&self, sentence: I) -> impl Iterator<Item = Segment<T, &V>>
    where
        I: IntoIterator<Item = T>,
    {
        let mut stack = sentence.into_iter().collect::<Vec<_>>();
        let total_len = stack.len();
        let mut r = 0;
        stack.reverse();

        std::iter::from_fn(move || {
            let l = r;
            r += 1;
            let first_token = stack.pop()?;
            let mut node = match self.root.get(&first_token) {
                Some(node) => node,
                None => return Some(Segment::Unmatched(first_token, l)),
            };
            let mut match_item = match &node.attr {
                Some(attr) => Segment::Match(attr, (l, r)),
                None => Segment::Unmatched(first_token, l),
            };
            for (i, token) in stack.iter().rev().enumerate() {
                if let Some(child) = node.children.get(token) {
                    if let Some(value) = &child.attr {
                        r = l + i + 2;
                        match_item = Segment::Match(value, (l, r));
                    }
                    node = child;
                } else {
                    break;
                }
            }
            stack.truncate(total_len - r);
            Some(match_item)
        })
    }
}

impl<T, V> Default for Fmm<T, V> {
    fn default() -> Self {
        Fmm {
            root: HashMap::new(),
        }
    }
}

impl<T, V> Fmm<T, V> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T, V> Default for Node<T, V> {
    fn default() -> Self {
        Node {
            attr: None,
            children: HashMap::new(),
        }
    }
}

pub trait StringSegmentation<V> {
    fn new() -> Self;

    fn add_phrase(&mut self, phrase: &str, attr: V);

    fn segmentation<'a>(&'a self, sentence: &str) -> impl Iterator<Item = Segment<char, &'a V>>
    where
        V: 'a;

    /// Split the sentence into substrings.
    fn split<'a, 'b>(&'a self, sentence: &'b str) -> impl Iterator<Item = &'b str> + 'a
    where
        'b: 'a,
        V: 'a,
    {
        let mut remaining = sentence;
        let mut segs = self.segmentation(sentence);
        std::iter::from_fn(move || {
            let len = match segs.next()? {
                Segment::Match(_, (l, r)) => r - l,
                Segment::Unmatched(_, _) => 1,
            };
            match remaining.char_indices().nth(len) {
                None => Some(remaining),
                Some((index, _)) => {
                    let (l, r) = remaining.split_at(index);
                    remaining = r;
                    Some(l)
                }
            }
        })
    }

    /// Convert each phrase in the sentence to the associated attribute.
    fn convert(&self, sentence: &str, separator: &str) -> String
    where
        V: Display,
    {
        let mut result = String::new();

        for seg in self.segmentation(sentence) {
            if !result.is_empty() {
                result.push_str(separator);
            }
            match seg {
                Segment::Unmatched(c, _) => result.push(c),
                Segment::Match(attr, _) => write!(result, "{attr}").unwrap(),
            }
        }

        result
    }
}

/// Forward maximum matching for string.
pub struct StringFmm<V> {
    fmm: Fmm<char, V>,
}

impl<V> StringSegmentation<V> for StringFmm<V> {
    fn new() -> Self {
        Self {
            fmm: Default::default(),
        }
    }

    fn add_phrase(&mut self, phrase: &str, attr: V) {
        self.fmm.add_phrase(phrase.chars(), attr);
    }

    fn segmentation<'a>(&'a self, sentence: &str) -> impl Iterator<Item = Segment<char, &'a V>>
    where
        V: 'a,
    {
        self.fmm.segmentation(sentence.chars())
    }
}

/// Backward maximum matching for string.
pub struct StringBmm<V> {
    bmm: Fmm<char, V>,
}

impl<V> StringSegmentation<V> for StringBmm<V> {
    fn new() -> Self {
        Self {
            bmm: Default::default(),
        }
    }

    fn add_phrase(&mut self, phrase: &str, attr: V) {
        self.bmm.add_phrase(phrase.chars().rev(), attr);
    }

    fn segmentation<'a>(&'a self, sentence: &str) -> impl Iterator<Item = Segment<char, &'a V>>
    where
        V: 'a,
    {
        let segs = self
            .bmm
            .segmentation(sentence.chars().rev())
            .collect::<Vec<_>>();
        segs.into_iter().rev()
    }
}

/// Bidirectional maximum matching for string.
#[derive(Default)]
pub struct StringBimm<V> {
    fmm: Fmm<char, V>,
    bmm: Fmm<char, V>,
}

impl<V: Clone> StringSegmentation<V> for StringBimm<V> {
    fn new() -> Self {
        Self {
            fmm: Default::default(),
            bmm: Default::default(),
        }
    }

    fn add_phrase(&mut self, phrase: &str, attr: V) {
        self.fmm.add_phrase(phrase.chars(), attr.clone());
        self.bmm.add_phrase(phrase.chars().rev(), attr);
    }

    fn segmentation<'a>(&'a self, sentence: &str) -> impl Iterator<Item = Segment<char, &'a V>>
    where
        V: 'a,
    {
        fn count_single<V>(segs: &[Segment<char, &V>]) -> usize {
            segs.iter()
                .filter(|item| match item {
                    Segment::Match(_, (l, r)) => r - l == 1,
                    Segment::Unmatched(_, _) => true,
                })
                .count()
        }

        let fmm_segs = self.fmm.segmentation(sentence.chars()).collect::<Vec<_>>();
        let bmm_segs = self
            .bmm
            .segmentation(sentence.chars().rev())
            .collect::<Vec<_>>();
        let segs = if fmm_segs.len() < bmm_segs.len()
            || (fmm_segs.len() == bmm_segs.len()
                && count_single(&fmm_segs) < count_single(&bmm_segs))
        {
            fmm_segs
        } else {
            let mut segs = bmm_segs;
            segs.reverse();
            segs
        };
        segs.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_token_conversion() {
        let mut fmm = Fmm::new();
        fmm.add_phrase("你".chars(), "ni");
        fmm.add_phrase("好".chars(), "hao");

        let mut result = fmm.segmentation("你好！".chars());
        assert_eq!(result.next().unwrap(), Segment::Match(&"ni", (0, 1)));
        assert_eq!(result.next().unwrap(), Segment::Match(&"hao", (1, 2)));
        assert_eq!(result.next().unwrap(), Segment::Unmatched('！', 2));
        assert!(result.next().is_none());
    }

    #[test]
    fn test_longest_match() {
        let mut fmm = StringFmm::new();
        fmm.add_phrase("乐", "le");
        fmm.add_phrase("乐趣", "le qu");
        fmm.add_phrase("乐曲", "yue qu");
        fmm.add_phrase("音乐", "yin yue");
        fmm.add_phrase("音音乐乐乐曲乐趣啊", "yin yin yue le yue qu le qu a");

        let result = fmm.convert("音音乐乐乐曲乐趣", " ");
        assert_eq!(result, "音 yin yue le yue qu le qu");

        let mut segs = fmm.segmentation("音乐的乐趣");
        assert_eq!(segs.next().unwrap(), Segment::Match(&"yin yue", (0, 2)));
        assert_eq!(segs.next().unwrap(), Segment::Unmatched('的', 2));
        assert_eq!(segs.next().unwrap(), Segment::Match(&"le qu", (3, 5)));
        assert!(segs.next().is_none());
        drop(segs);

        fmm.add_phrase("音音乐乐乐曲乐趣", "yin yin yue le yue qu le qu");
        let result = fmm.segmentation("音音乐乐乐曲乐趣");
        assert_eq!(result.count(), 1);
    }

    #[test]
    fn test_longest_match_2() {
        let mut fmm = StringFmm::new();
        fmm.add_phrase("一", "1");
        fmm.add_phrase("四", "4");
        fmm.add_phrase("五", "5");
        fmm.add_phrase("十", "10");
        fmm.add_phrase("十一万", "110000");
        fmm.add_phrase("四千五百", "4500");
        fmm.add_phrase("五百一十四", "514");
        fmm.add_phrase("四十", "40");
        fmm.add_phrase("十一万四千五百一十四", "114514");
        assert_eq!(
            fmm.convert("四千五百一十五十一万四千五百一十四十一万四千五百四十四", ""),
            "450011051145141100004500404"
        );
    }

    #[test]
    fn test_fmm_bmm_bimm() {
        const DICT: &[&str] = &[
            "我们",
            "在野",
            "生动",
            "野生动物园",
            "野生",
            "动物",
            "动物园",
            "做好",
            "奥运",
            "运动员",
            "动员",
            "工作",
        ];

        let mut fmm = StringFmm::new();
        let mut bmm = StringBmm::new();
        let mut bimm = StringBimm::new();

        for phrase in DICT {
            fmm.add_phrase(phrase, ());
            bmm.add_phrase(phrase, ());
            bimm.add_phrase(phrase, ());
        }

        assert_eq!(
            fmm.split("我们在野生动物园玩").collect::<Vec<_>>(),
            vec!["我们", "在野", "生动", "物", "园", "玩"]
        );
        assert_eq!(
            bmm.split("我们在野生动物园玩").collect::<Vec<_>>(),
            vec!["我们", "在", "野生动物园", "玩"]
        );
        assert_eq!(
            bimm.split("我们在野生动物园玩").collect::<Vec<_>>(),
            vec!["我们", "在", "野生动物园", "玩"]
        );

        assert_eq!(
            fmm.split("做好奥运动员工作").collect::<Vec<_>>(),
            vec!["做好", "奥运", "动员", "工作"]
        );
        assert_eq!(
            bmm.split("做好奥运动员工作").collect::<Vec<_>>(),
            vec!["做好", "奥", "运动员", "工作"]
        );
        assert_eq!(
            bimm.split("做好奥运动员工作").collect::<Vec<_>>(),
            vec!["做好", "奥运", "动员", "工作"]
        );
    }
}
