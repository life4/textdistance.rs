//! Length distance
use crate::{Algorithm, Result};

/// Length distance is the absolute difference between the lengths of the two sequences.
///
/// It's a very dumb algorithm that says that "qwer" and "zxcv" are the same.
/// Still, it works surprisingly well in some specific scenarios, especially on big
/// sequences.
#[derive(Default)]
pub struct Length {}

impl Algorithm<usize> for Length {
    fn for_iter<C, E>(&self, s1: C, s2: C) -> Result<usize>
    where
        C: Iterator<Item = E>,
        E: Eq,
    {
        let l1 = s1.count();
        let l2 = s2.count();
        Result {
            abs: l1.abs_diff(l2),
            is_distance: true,
            max: l1.max(l2),
            len1: l1,
            len2: l2,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::str::length;
    use assert2::assert;
    use rstest::rstest;

    #[rstest]
    #[case("", "", 0)]
    #[case("", "a", 1)]
    #[case("a", "", 1)]
    #[case("a", "a", 0)]
    #[case("a", "b", 0)]
    #[case("abcde", "abcef", 0)]
    #[case("abcde", "abcfde", 1)]
    #[case("abcd", "bcd", 1)]
    #[case("ab", "cdefg", 3)]
    fn function_str(#[case] s1: &str, #[case] s2: &str, #[case] exp: usize) {
        assert!(length(s1, s2) == exp);
    }
}
