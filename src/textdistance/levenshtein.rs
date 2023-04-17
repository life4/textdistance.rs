use super::algorithm::Algorithm;

pub struct Levenshtein {}

impl Levenshtein {
    fn from_str(&self, s1: &str, s2: &str) -> usize {
        self.from_iter(s1.chars(), s2.chars())
    }

    fn from_iter<C, E>(&self, s1: C, s2: C) -> usize
    where
        C: Iterator<Item = E> + Clone,
        E: Eq,
    {
        let s1_len = s1.to_owned().count();
        let s2_len = s2.to_owned().count();
        if s1_len == 0 {
            return s2_len;
        }
        if s2_len == 0 {
            return s1_len;
        }

        let mut cache: Vec<usize> = (1..).take(s1_len).collect();
        let mut s1_dist;
        let mut s2_dist;

        let mut result = 0;
        for (s2_i, s2_char) in s2.enumerate() {
            result = s2_i;
            s1_dist = s2_i;

            for (s1_i, s1_char) in s1.to_owned().enumerate() {
                s2_dist = if s1_char == s2_char {
                    s1_dist
                } else {
                    s1_dist + 1
                };

                s1_dist = cache[s1_i];
                result = if s1_dist > result {
                    if s2_dist > result {
                        result + 1
                    } else {
                        s2_dist
                    }
                } else if s2_dist > s1_dist {
                    s1_dist + 1
                } else {
                    s2_dist
                };

                cache[s1_i] = result;
            }
        }

        result
    }
}

impl Algorithm for Levenshtein {
    fn distance(&self, s1: &str, s2: &str) -> usize {
        self.from_str(s1, s2)
    }
}

const DEFAULT: Levenshtein = Levenshtein {};

pub fn levenshtein(s1: &str, s2: &str) -> usize {
    DEFAULT.from_str(s1, s2)
}

#[cfg(test)]
mod tests {
    use super::*;
    // use proptest::prelude::*;

    #[test]
    fn function() {
        let f = levenshtein;
        assert_eq!(f("", ""), 0);
        assert_eq!(f("", "\0"), 1);
        assert_eq!(f("", "abc"), 3);
        assert_eq!(f("abc", ""), 3);
        assert_eq!(f("sitting", "sitting"), 0);
        assert_eq!(f("sitting", "kitten"), 3);
        assert_eq!(f("example", "samples"), 3);
        assert_eq!(f("distance", "difference"), 5);
        assert_eq!(f("test", "text"), 1);
        assert_eq!(f("test", "tset"), 2);
        assert_eq!(f("test", "qwe"), 4);
        assert_eq!(f("test", "testit"), 2);
        assert_eq!(f("test", "tesst"), 1);
        assert_eq!(f("test", "tet"), 1);
    }
}
