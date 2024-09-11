//! Longest common subsequence
use crate::{Algorithm, Result};
use alloc::vec;
use alloc::vec::Vec;

/// The length of the [Longest common subsequence].
///
/// It differs from the [`LCSStr`](crate::LCSStr). Unlike substrings, subsequences are not required
/// to occupy consecutive positions within the original sequences.
///
/// [Longest common subsequence]: https://en.wikipedia.org/wiki/Longest_common_subsequence
#[derive(Default)]
pub struct LCSSeq {}

impl Algorithm<usize> for LCSSeq {
    fn for_vec<E: Eq>(&self, s1: &[E], s2: &[E]) -> Result<usize> {
        let l1 = s1.len();
        let l2 = s2.len();
        let mut lengths = vec![vec![0; l2 + 1]; l1 + 1];

        for (i, char1) in s1.iter().enumerate() {
            for (j, char2) in s2.iter().enumerate() {
                lengths[i + 1][j + 1] = if char1 == char2 {
                    lengths[i][j] + 1
                } else {
                    lengths[i + 1][j].max(lengths[i][j + 1])
                };
            }
        }

        let mut result = Vec::<&E>::new();
        let mut i = l1;
        let mut j = l2;
        while i != 0 && j != 0 {
            if lengths[i][j] == lengths[i - 1][j] {
                i -= 1;
            } else if lengths[i][j] == lengths[i][j - 1] {
                j -= 1;
            } else {
                assert!(s1[i - 1] == s2[j - 1]);
                result.push(&s1[i - 1]);
                i -= 1;
                j -= 1;
            }
        }
        // val: Some(result.into_iter().rev().collect::<String>())
        Result {
            abs: result.len(),
            is_distance: false,
            max: l1.max(l2),
            len1: l1,
            len2: l2,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::str::lcsseq;
    use assert2::assert;
    use proptest::prelude::*;
    use rstest::rstest;

    #[rstest]
    #[case("", "", 0)]
    #[case("", "abcd", 0)]
    #[case("abcd", "", 0)]
    #[case("ab", "cd", 0)]
    #[case("abcd", "abcd", 4)] // "abcd"
    #[case("test", "text", 3)] // "tet"
    #[case("thisisatest", "testing123testing", 7)] // "tsitest"
    #[case("abcd", "c", 1)] // "c"
    #[case("abcd", "d", 1)] // "d"
    #[case("abcd", "e", 0)] // ""
    #[case("abcdefghi", "acegi", 5)] // "acegi"
    #[case("abcdgh", "aedfhr", 3)] // "adh"
    #[case("aggtab", "gxtxayb", 4)] // "gtab"
    #[case("你好，世界", "再见世界", 2)] // "世界"
    fn function_str(#[case] s1: &str, #[case] s2: &str, #[case] exp: usize) {
        assert!(lcsseq(s1, s2) == exp);
    }

    proptest! {
        #[test]
        fn prop(s1 in ".*", s2 in ".*") {
            let res = lcsseq(&s1, &s2);
            let res2 = lcsseq(&s2, &s1);
            prop_assert_eq!(res, res2);
            prop_assert!(res <= s1.len() || res <= s2.len());
        }
    }
}
