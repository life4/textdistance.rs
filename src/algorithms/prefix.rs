//! Prefix similarity
use crate::{Algorithm, Result};

/// Prefix similarity is the length of the longest common prefix for the given sequences.
///
/// It's a very dumb metric but it can be surprisingly effective for comparing words
/// in languages with an extensive use of [suffixes](https://en.wikipedia.org/wiki/Suffix).
#[derive(Default)]
pub struct Prefix {}

impl Algorithm<usize> for Prefix {
    fn for_iter<C, E>(&self, mut s1: C, mut s2: C) -> Result<usize>
    where
        C: Iterator<Item = E>,
        E: Eq,
    {
        let mut result = 0;
        let mut prev_match: bool = true;
        let mut l1 = 0;
        let mut l2 = 0;
        loop {
            match (s1.next(), s2.next()) {
                (Some(c1), Some(c2)) => {
                    l1 += 1;
                    l2 += 1;
                    if c1 != c2 {
                        prev_match = false;
                    } else if prev_match {
                        result += 1;
                    }
                }
                (Some(_), None) => {
                    l1 += 1;
                }
                (None, Some(_)) => {
                    l2 += 1;
                }
                (None, None) => {
                    break;
                }
            }
        }
        Result {
            abs: result,
            is_distance: false,
            max: l1.max(l2),
            len1: l1,
            len2: l2,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::str::prefix;
    use assert2::assert;
    use rstest::rstest;

    #[rstest]
    #[case("", "", 0)]
    #[case("", "a", 0)]
    #[case("a", "", 0)]
    #[case("a", "a", 1)]
    #[case("a", "b", 0)]
    #[case("abcde", "abcef", 3)]
    #[case("abcde", "abcfde", 3)]
    #[case("abcd", "bcd", 0)]
    fn function_str(#[case] s1: &str, #[case] s2: &str, #[case] exp: usize) {
        assert!(prefix(s1, s2) == exp);
    }
}
