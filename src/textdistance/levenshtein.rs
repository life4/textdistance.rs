use super::algorithm::Algorithm;

pub struct Levenshtein {}

impl Levenshtein {
    fn from_str(&self, s1: &str, s2: &str) -> usize {
        self.from_iter(s1.chars(), s2.chars())
    }

    fn from_iter<C, E>(&self, s1: C, s2: C) -> usize
    where
        C: Iterator<Item = E>,
        E: Eq,
    {
        let s1: Vec<E> = s1.collect();
        let s1_len = s1.len();
        if s1_len == 0 {
            return s2.count();
        }

        let mut cache: Vec<usize> = (1..).take(s1_len).collect();
        let mut dist1;
        let mut dist2;

        let mut result = 0;
        let mut s2_empty = true;
        for (i2, c2) in s2.enumerate() {
            result = i2;
            dist1 = i2;
            s2_empty = false;

            for (i1, c1) in s1.iter().enumerate() {
                dist2 = if c1 == &c2 { dist1 } else { dist1 + 1 };
                dist1 = cache[i1];
                result = if dist1 > result {
                    if dist2 > result {
                        result + 1
                    } else {
                        dist2
                    }
                } else if dist2 > dist1 {
                    dist1 + 1
                } else {
                    dist2
                };
                cache[i1] = result;
            }
        }
        if s2_empty {
            return s1_len;
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
