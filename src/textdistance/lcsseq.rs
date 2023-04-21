use super::algorithm::{Algorithm, Result};

pub struct LCSSeq {}

impl Algorithm for LCSSeq {
    fn for_iter<C, E>(&self, s1: C, s2: C) -> Result
    where
        C: Iterator<Item = E>,
        E: Eq + Copy,
    {
        let s1: Vec<E> = s1.collect();
        let s2: Vec<E> = s2.collect();
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

        let mut result = Vec::<E>::new();
        let mut i = l1;
        let mut j = l2;
        while i != 0 && j != 0 {
            if lengths[i][j] == lengths[i - 1][j] {
                i -= 1;
            } else if lengths[i][j] == lengths[i][j - 1] {
                j -= 1;
            } else {
                assert!(s1[i - 1] == s2[j - 1]);
                result.push(s1[i - 1]);
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

const DEFAULT: LCSSeq = LCSSeq {};

pub fn lcsseq(s1: &str, s2: &str) -> usize {
    DEFAULT.for_str(s1, s2).sim()
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn basic() {
        let f = lcsseq;
        assert_eq!(f("", ""), 0);
        assert_eq!(f("", "abcd"), 0);
        assert_eq!(f("abcd", ""), 0);
        assert_eq!(f("ab", "cd"), 0);
        assert_eq!(f("abcd", "abcd"), 4); // "abcd"
        assert_eq!(f("test", "text"), 3); // "tet"
        assert_eq!(f("thisisatest", "testing123testing"), 7); // "tsitest"
        assert_eq!(f("abcd", "c"), 1); // "c"
        assert_eq!(f("abcd", "d"), 1); // "d"
        assert_eq!(f("abcd", "e"), 0); // ""
        assert_eq!(f("abcdefghi", "acegi"), 5); // "acegi"
        assert_eq!(f("abcdgh", "aedfhr"), 3); // "adh"
        assert_eq!(f("aggtab", "gxtxayb"), 4); // "gtab"
        assert_eq!(f("你好，世界", "再见世界"), 2); // "世界"
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
