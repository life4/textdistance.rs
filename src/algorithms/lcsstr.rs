//! Longest common substring
use crate::{Algorithm, Result};
use alloc::vec;

/// The length of the [Longest common substring].
///
/// A longest common substring of two or more strings is a longest string
/// that is a substring of all of them.
///
/// [Longest common substring]: https://en.wikipedia.org/wiki/Longest_common_substring
#[derive(Default)]
pub struct LCSStr {}

impl Algorithm<usize> for LCSStr {
    fn for_vec<E: Eq>(&self, s1: &[E], s2: &[E]) -> Result<usize> {
        let l1 = s1.len();
        let l2 = s2.len();
        let mut dp = vec![vec![0; l2 + 1]; l1 + 1];
        // let mut result_end = 0;
        let mut result_len = 0;
        for (i, c1) in s1.iter().enumerate() {
            for (j, c2) in s2.iter().enumerate() {
                if c1 == c2 {
                    let new_len = dp[i][j] + 1;
                    dp[i + 1][j + 1] = new_len;
                    if new_len > result_len {
                        result_len = new_len;
                        // result_end = i + 1;
                    };
                }
            }
        }
        // s1[(result_end - result_len)..result_end].to_vec()
        Result {
            abs: result_len,
            is_distance: false,
            max: l1.max(l2),
            len1: l1,
            len2: l2,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::str::lcsstr;
    use assert2::assert;
    use proptest::prelude::*;
    use rstest::rstest;

    #[rstest]
    #[case("", "", "")]
    #[case("a", "", "")]
    #[case("", "a", "")]
    #[case("a", "a", "a")]
    #[case("ab", "b", "b")]
    #[case("abcdef", "bcd", "bcd")]
    #[case("bcd", "abcdef", "bcd")]
    #[case("abcdef", "xabded", "ab")]
    #[case("GeeksforGeeks", "GeeksQuiz", "Geeks")]
    #[case("abcdxyz", "xyzabcd", "abcd")]
    #[case("zxabcdezy", "yzabcdezx", "abcdez")]
    #[case("OldSite:GeeksforGeeks.org", "NewSite:GeeksQuiz.com", "Site:Geeks")]
    fn function_str(#[case] s1: &str, #[case] s2: &str, #[case] exp: &str) {
        assert!(lcsstr(s1, s2) == exp.len());
    }

    #[test]
    fn unicode() {
        let f = lcsstr;
        assert!(f("п", "") == 0);
        assert!(f("", "п") == 0);
        assert!(f("п", "п") == 1);
        assert!(f("привет", "пока") == 1);
        assert!(f("корвет", "привет") == 3);
    }

    proptest! {
        #[test]
        fn prop(s1 in ".*", s2 in ".*") {
            let res = lcsstr(&s1, &s2);
            let res2 = lcsstr(&s2, &s1);
            prop_assert_eq!(res, res2);
            prop_assert!(res <= s1.len() || res <= s2.len());
        }
    }
}
