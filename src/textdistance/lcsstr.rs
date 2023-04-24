use super::algorithm::{Algorithm, Result};

#[derive(Default)]
pub struct LCSStr {}

impl Algorithm for LCSStr {
    fn for_vec<E: Eq>(&self, s1: &[E], s2: &[E]) -> Result {
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
    use crate::textdistance::str::lcsstr;
    use proptest::prelude::*;

    #[test]
    fn basic() {
        let f = lcsstr;
        assert_eq!(f("", ""), 0);
        assert_eq!(f("a", ""), 0);
        assert_eq!(f("", "a"), 0);
        assert_eq!(f("a", "a"), 1); // "a"
        assert_eq!(f("ab", "b"), 1); // "b"
        assert_eq!(f("abcdef", "bcd"), 3); // "bcd"
        assert_eq!(f("bcd", "abcdef"), 3); // "bcd"
        assert_eq!(f("abcdef", "xabded"), 2); // "ab"
        assert_eq!(f("GeeksforGeeks", "GeeksQuiz"), 5); // "Geeks"
        assert_eq!(f("abcdxyz", "xyzabcd"), 4); // "abcd"
        assert_eq!(f("zxabcdezy", "yzabcdezx"), 6); // "abcdez"
        assert_eq!(f("OldSite:GeeksforGeeks.org", "NewSite:GeeksQuiz.com"), 10);
        // "Site:Geeks"
    }

    #[test]
    fn unicode() {
        let f = lcsstr;
        assert_eq!(f("п", ""), 0);
        assert_eq!(f("", "п"), 0);
        assert_eq!(f("п", "п"), 1);
        assert_eq!(f("привет", "пока"), 1);
        assert_eq!(f("корвет", "привет"), 3);
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
