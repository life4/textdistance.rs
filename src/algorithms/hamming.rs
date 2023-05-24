//! Hamming distance
use crate::{Algorithm, Result};

/// [Hamming distance] is the number of positions at which the corresponding symbols are different.
///
/// [Hamming distance]: https://en.wikipedia.org/wiki/Hamming_distance
#[derive(Default)]
pub struct Hamming {
    /// If false (default), the longer strings is truncated to the same length
    /// as the shorter one.
    pub truncate: bool,
}

impl Algorithm<usize> for Hamming {
    fn for_iter<C, E>(&self, mut s1: C, mut s2: C) -> Result<usize>
    where
        C: Iterator<Item = E>,
        E: Eq,
    {
        let mut result = 0;
        let mut l1 = 0;
        let mut l2 = 0;
        loop {
            match (s1.next(), s2.next()) {
                (Some(c1), Some(c2)) => {
                    l1 += 1;
                    l2 += 1;
                    if c1 != c2 {
                        result += 1;
                    }
                }
                (Some(_), None) => {
                    l1 += 1;
                    if !self.truncate {
                        result += 1;
                    }
                }
                (None, Some(_)) => {
                    l2 += 1;
                    if !self.truncate {
                        result += 1;
                    }
                }
                (None, None) => {
                    break;
                }
            }
        }
        Result {
            abs: result,
            is_distance: true,
            max: l1.max(l2),
            len1: l1,
            len2: l2,
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::float_cmp)]

    use super::{Algorithm, Hamming};
    use crate::str::hamming;
    use assert2::assert;
    use proptest::prelude::*;
    use rstest::rstest;

    #[rstest]
    #[case("", "", 0)]
    #[case("", "\0", 1)]
    #[case("", "abc", 3)]
    #[case("abc", "", 3)]
    #[case("sitting", "sitting", 0)]
    #[case("abcdefg", "hijklmn", 7)]
    #[case("karolin", "kathrin", 3)]
    #[case("hello", "world", 4)]
    #[case("Rust", "rust", 1)]
    #[case("hi mark", "hi markus", 2)]
    fn function_str(#[case] s1: &str, #[case] s2: &str, #[case] exp: usize) {
        assert!(hamming(s1, s2) == exp);
    }

    #[test]
    fn default_struct_result() {
        let r = Hamming::default().for_str("Rust", "rust");
        assert!(r.dist() == 1);
        assert!(r.sim() == 3);
        assert!(r.ndist() == 0.25);
    }

    #[test]
    fn truncate() {
        let a = Hamming { truncate: true };
        assert!(a.for_str("hi mark", "hi markus").val() == 0);
        assert!(a.for_str("Hi mark", "hi markus").val() == 1);
    }

    proptest! {
        #[test]
        fn prop(s1 in ".*", s2 in ".*") {
            let res = hamming(&s1, &s2);
            let res2 = hamming(&s2, &s1);
            prop_assert_eq!(res, res2);
            prop_assert!(res <= s1.len() || res <= s2.len());
        }
    }
}
