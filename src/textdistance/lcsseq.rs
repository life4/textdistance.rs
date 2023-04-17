use super::algorithm::Algorithm;
use ndarray::Array2;

pub struct LCSSeq {}

impl LCSSeq {
    fn from_str(&self, s1: &str, s2: &str) -> Vec<char> {
        self.from_iterator(s1.chars(), s2.chars())
    }

    fn from_iterator<C, E>(&self, s1: C, s2: C) -> Vec<E>
    where
        C: Iterator<Item = E> + Clone,
        E: Eq + Copy,
    {
        let s1_len = s1.to_owned().count();
        let s2_len = s2.to_owned().count();
        let mut lengths = Array2::from_elem((s1_len + 1, s2_len + 1), 0);
        let s1vec: Vec<E> = s1.collect();

        for (i, char1) in s1vec.iter().enumerate() {
            for (j, char2) in s2.to_owned().enumerate() {
                lengths[[i + 1, j + 1]] = if char1 == &char2 {
                    lengths[[i, j]] + 1
                } else {
                    lengths[[i + 1, j]].max(lengths[[i, j + 1]])
                };
            }
        }

        let mut result = Vec::<E>::new();
        let mut i = s1_len;
        let mut j = s2_len;
        while i != 0 && j != 0 {
            if lengths[[i, j]] == lengths[[i - 1, j]] {
                i -= 1;
            } else if lengths[[i, j]] == lengths[[i, j - 1]] {
                j -= 1;
            } else {
                // assert s1[i - 1] == s2[j - 1]
                result.push(s1vec[i - 1]);
                i -= 1;
                j -= 1;
            }
        }
        result.into_iter().rev().collect()
    }
}

impl Algorithm for LCSSeq {
    fn similarity(&self, s1: &str, s2: &str) -> usize {
        self.from_str(s1, s2).len()
    }
}

const DEFAULT: LCSSeq = LCSSeq {};

pub fn lcsseq(s1: &str, s2: &str) -> String {
    DEFAULT.from_str(s1, s2).into_iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn basic() {
        let f = lcsseq;
        assert_eq!(f("ab", "cd"), "");
        assert_eq!(f("abcd", "abcd"), "abcd");
        assert_eq!(f("test", "text"), "tet");
        assert_eq!(f("thisisatest", "testing123testing"), "tsitest");
        assert_eq!(f("", ""), "");
        assert_eq!(f("", "abcd"), "");
        assert_eq!(f("abcd", ""), "");
        assert_eq!(f("abcd", "c"), "c");
        assert_eq!(f("abcd", "d"), "d");
        assert_eq!(f("abcd", "e"), "");
        assert_eq!(f("abcdefghi", "acegi"), "acegi");
        assert_eq!(f("abcdgh", "aedfhr"), "adh");
        assert_eq!(f("aggtab", "gxtxayb"), "gtab");
        assert_eq!(f("你好，世界", "再见世界"), "世界");
    }

    proptest! {
        #[test]
        fn prop(s1 in ".*", s2 in ".*") {
            let res = lcsseq(&s1, &s2);
            prop_assert!(res.len() <= s1.len() || res.len() <= s2.len());
        }
    }
}
