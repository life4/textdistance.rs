//! Suffix similarity
use crate::{Algorithm, Result};

/// Suffix similarity is the length of the longest common suffix of the given sequences.
///
/// It's a very dumb metric but it can work surprisingly well for comparing words
/// in languages with an active use of [prefixes](https://en.wikipedia.org/wiki/Prefix).
#[derive(Default)]
pub struct Suffix {}

impl Algorithm<usize> for Suffix {
    fn for_vec<E: Eq>(&self, s1: &[E], s2: &[E]) -> Result<usize> {
        let mut result = 0;
        for (c1, c2) in s1.iter().rev().zip(s2.iter().rev()) {
            if c1 == c2 {
                result += 1;
            } else {
                break;
            }
        }
        let l1 = s1.len();
        let l2 = s2.len();
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
    use crate::str::suffix;
    use assert2::assert;
    use rstest::rstest;

    #[rstest]
    #[case("", "", 0)]
    #[case("", "a", 0)]
    #[case("a", "", 0)]
    #[case("a", "a", 1)]
    #[case("a", "b", 0)]
    #[case("abcde", "abcef", 0)]
    #[case("abcde", "abfcde", 3)]
    #[case("abcd", "fabcd", 4)]

    fn function_str(#[case] s1: &str, #[case] s2: &str, #[case] exp: usize) {
        assert!(suffix(s1, s2) == exp);
    }
}
