//! Gestalt pattern matching
use crate::{Algorithm, Result};
use alloc::vec;
use alloc::vec::Vec;

/// [Ratcliff/Obershelp similarity] is [`LCSStr`] that recursively finds matches
/// on both sides of the longest substring.
///
/// The non-normalized result is a double number of matching characters defined as the first
/// longest common substring plus recursively the number of matching characters in
/// the non-matching regions on both sides of the longest common substring.
///
/// The normalized result is the non-normalized one divided by the sum of the input string lengths.
///
/// [Ratcliff/Obershelp similarity]: https://en.wikipedia.org/wiki/Gestalt_pattern_matching
/// [`LCSStr`]: crate::LCSStr
#[derive(Default)]
pub struct RatcliffObershelp {}

impl Algorithm<usize> for RatcliffObershelp {
    fn for_vec<E: Eq>(&self, s1: &[E], s2: &[E]) -> Result<usize> {
        let l1 = s1.len();
        let l2 = s2.len();
        let mut stack: Vec<((usize, usize), (usize, usize))> = Vec::new();
        stack.push(((0, l1), (0, l2)));
        let mut result = 0;

        while let Some(top) = stack.pop() {
            let ((part1_start, part1_len), (part2_start, part2_len)) = top;
            let s1_part = s1[part1_start..(part1_start + part1_len)].iter();
            let s2_part: Vec<&E> = s2[part2_start..(part2_start + part2_len)].iter().collect();

            let mut dp = vec![vec![0; l2 + 1]; l1 + 1];
            let mut prefix1_end = 0;
            let mut prefix2_end = 0;
            let mut match_len: usize = 0;
            for (i1, c1) in s1_part.enumerate() {
                for (i2, c2) in s2_part.iter().enumerate() {
                    if &c1 == c2 {
                        let new_len: usize = dp[i1][i2] + 1;
                        dp[i1 + 1][i2 + 1] = new_len;
                        if new_len > match_len {
                            debug_assert!(i1 + 1 >= new_len);
                            debug_assert!(i2 + 1 >= new_len);
                            match_len = new_len;
                            prefix1_end = i1 + 1;
                            prefix2_end = i2 + 1;
                        };
                    }
                }
            }

            if match_len != 0 {
                let prefix1_len = prefix1_end - match_len;
                let prefix2_len = prefix2_end - match_len;
                if prefix1_len != 0 && prefix2_len != 0 {
                    stack.push(((part1_start, prefix1_len), (part2_start, prefix2_len)));
                }
                let suffix1_len = part1_len - prefix1_end;
                let suffix2_len = part2_len - prefix2_end;
                if suffix1_len != 0 && suffix2_len != 0 {
                    stack.push((
                        (part1_start + prefix1_end, suffix1_len),
                        (part2_start + prefix2_end, suffix2_len),
                    ));
                }
                result += match_len;
            }
        }

        Result {
            abs: 2 * result,
            is_distance: false,
            max: l1 + l2,
            len1: l1,
            len2: l2,
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::float_cmp)]

    use super::{Algorithm, RatcliffObershelp};
    use crate::str::ratcliff_obershelp;
    use assert2::assert;
    use rstest::rstest;

    #[rstest]
    #[case("", "", 1.)]
    #[case("abc", "", 0.)]
    #[case("", "abc", 0.)]
    #[case("abc", "abc", 1.)]
    fn function_str(#[case] s1: &str, #[case] s2: &str, #[case] exp: f64) {
        assert!(ratcliff_obershelp(s1, s2) == exp);
    }

    #[test]
    fn default_abs() {
        let a = RatcliffObershelp::default();
        assert!(
            a.for_str("GESTALT PATTERN MATCHING", "GESTALT PRACTICE")
                .val()
                == 24
        );
        assert!(
            a.for_str("GESTALT PRACTICE", "GESTALT PATTERN MATCHING")
                .val()
                == 26
        );
    }
}
