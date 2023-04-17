use super::algorithm::Algorithm;

pub struct Hamming {}

impl Hamming {
    fn from_str(&self, s1: &str, s2: &str) -> usize {
        self.from_iter(s1.chars(), s2.chars())
    }

    fn from_iter<C, E>(&self, s1: C, s2: C) -> usize
    where
        C: Iterator<Item = E> + Clone,
        E: Eq,
    {
        let mut result = 0;
        for (s1_char, s2_char) in s1.to_owned().zip(s2.to_owned()) {
            if s1_char != s2_char {
                result += 1
            }
        }
        let s1_len = s1.count();
        let s2_len = s2.count();
        result + s1_len.abs_diff(s2_len)
    }
}

impl Algorithm for Hamming {
    fn distance(&self, s1: &str, s2: &str) -> usize {
        self.from_str(s1, s2)
    }
}

const DEFAULT: Hamming = Hamming {};

pub fn hamming(s1: &str, s2: &str) -> usize {
    DEFAULT.from_str(s1, s2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn function() {
        let f = hamming;
        assert_eq!(f("", ""), 0);
        assert_eq!(f("", "\0"), 1);
        assert_eq!(f("", "abc"), 3);
        assert_eq!(f("abc", ""), 3);
        assert_eq!(f("sitting", "sitting"), 0);
        assert_eq!(f("abcdefg", "hijklmn"), 7);
        assert_eq!(f("karolin", "kathrin"), 3);
        assert_eq!(f("hello", "world"), 4);
        assert_eq!(f("Rust", "rust"), 1);
        assert_eq!(f("hi mark", "hi markus"), 2);
    }

    #[test]
    fn default_struct() {
        assert_eq!(DEFAULT.distance("Rust", "rust"), 1);
        assert_eq!(DEFAULT.similarity("Rust", "rust"), 3);
        assert_eq!(DEFAULT.maximum("Rust", "rust"), 4);
    }

    proptest! {
        #[test]
        fn prop(s1 in ".*", s2 in ".*") {
            let res = hamming(&s1, &s2);
            prop_assert!(res <= s1.len() || res <= s2.len());
        }
    }
}
