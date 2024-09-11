//! Damerau-Levenshtein distance
#![cfg(feature = "std")]
use crate::{Algorithm, Result};
use alloc::vec;
use alloc::vec::Vec;
use core::hash::Hash;
use std::collections::HashMap;

/// [Damerau-Levenshtein distance] is an edit distance between two sequences.
///
/// It is an improved version of [Levenshtein](crate::Levenshtein) that also includes
/// transpositions.
///
/// It is the minimum number of operations (consisting of insertions, deletions or
/// substitutions of a single character, or transposition of two adjacent characters)
/// required to change one text into the other.
///
/// [Damerau-Levenshtein distance]: https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance
pub struct DamerauLevenshtein {
    /// If false (default), allow adjacent transpositions.
    pub restricted: bool,

    /// The cost of removing a character.
    pub del_cost: usize,

    /// The cost of adding a new character.
    pub ins_cost: usize,

    /// The cost of replacing a character with another one.
    pub sub_cost: usize,

    /// The cost of swapping two adjacent characters.
    pub trans_cost: usize,
}

impl Default for DamerauLevenshtein {
    fn default() -> Self {
        Self {
            restricted: false,
            del_cost: 1,
            ins_cost: 1,
            sub_cost: 1,
            trans_cost: 1,
        }
    }
}

impl DamerauLevenshtein {
    fn get_unrestricted<E: Eq + Hash>(&self, s1: &[E], s2: &[E]) -> Result<usize> {
        let l1 = s1.len();
        let l2 = s2.len();
        let max_dist = l2 + l1;

        let mut mat: Vec<Vec<usize>> = vec![vec![0; l2 + 2]; l1 + 2];
        mat[0][0] = max_dist;
        for i in 0..=l1 {
            mat[i + 1][0] = max_dist;
            mat[i + 1][1] = i;
        }
        for i in 0..=l2 {
            mat[0][i + 1] = max_dist;
            mat[1][i + 1] = i;
        }

        let mut char_map: HashMap<&E, usize> = HashMap::new();
        for (i1, c1) in s1.iter().enumerate() {
            let mut db = 0;
            let i1 = i1 + 1;

            for (i2, c2) in s2.iter().enumerate() {
                let i2 = i2 + 1;
                let last = *char_map.get(&c2).unwrap_or(&0);

                let sub_cost = if c1 == c2 { 0 } else { self.sub_cost };
                mat[i1 + 1][i2 + 1] = min4(
                    mat[i1][i2] + sub_cost,                                    // substitution
                    mat[i1 + 1][i2] + self.del_cost,                           // deletion
                    mat[i1][i2 + 1] + self.ins_cost,                           // insertion
                    mat[last][db] + i1 + i2 - 2 + self.trans_cost - last - db, // transposition
                );

                if c1 == c2 {
                    db = i2;
                }
            }

            char_map.insert(c1, i1);
        }

        Result {
            is_distance: true,
            abs: mat[l1 + 1][l2 + 1],
            max: l1.max(l2),
            len1: l1,
            len2: l2,
        }
    }

    #[allow(clippy::needless_range_loop)]
    fn get_restricted<E: Eq + Hash>(&self, s1: &[E], s2: &[E]) -> Result<usize> {
        let l1 = s1.len();
        let l2 = s2.len();

        let mut mat: Vec<Vec<usize>> = vec![vec![0; l2 + 2]; l1 + 2];
        for i in 0..=l1 {
            mat[i][0] = i;
        }
        for i in 0..=l2 {
            mat[0][i] = i;
        }

        for (i1, c1) in s1.iter().enumerate() {
            for (i2, c2) in s2.iter().enumerate() {
                let sub_cost = if c1 == c2 { 0 } else { self.sub_cost };
                mat[i1 + 1][i2 + 1] = min3(
                    mat[i1][i2 + 1] + self.del_cost, // deletion
                    mat[i1 + 1][i2] + self.ins_cost, // insertion
                    mat[i1][i2] + sub_cost,          // substitution
                );

                // transposition
                if i1 == 0 || i2 == 0 {
                    continue;
                };
                if c1 != &s2[i2 - 1] {
                    continue;
                };
                if &s1[i1 - 1] != c2 {
                    continue;
                };
                let trans_cost = if c1 == c2 { 0 } else { self.trans_cost };
                mat[i1 + 1][i2 + 1] = mat[i1 + 1][i2 + 1].min(mat[i1 - 1][i2 - 1] + trans_cost);
            }
        }

        Result {
            is_distance: true,
            abs: mat[l1][l2],
            max: l1.max(l2),
            len1: l1,
            len2: l2,
        }
    }
}

impl Algorithm<usize> for DamerauLevenshtein {
    fn for_vec<E: Eq + Hash>(&self, s1: &[E], s2: &[E]) -> Result<usize> {
        if self.restricted {
            self.get_restricted(s1, s2)
        } else {
            self.get_unrestricted(s1, s2)
        }
    }
}

fn min4(a: usize, b: usize, c: usize, d: usize) -> usize {
    a.min(b).min(c).min(d)
}

fn min3(a: usize, b: usize, c: usize) -> usize {
    a.min(b).min(c)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::str::{damerau_levenshtein, damerau_levenshtein_restricted};
    use assert2::assert;
    use proptest::prelude::*;
    use rstest::rstest;

    #[rstest]
    #[case("", "", 0)]
    #[case("", "\0", 1)]
    #[case("", "abc", 3)]
    #[case("abc", "", 3)]
    #[case("hannah", "hannha", 1)]
    #[case("FOO", "BOR", 2)]
    #[case("BAR", "BOR", 1)]
    #[case("hansi", "hasni", 1)]
    #[case("zzaabbio", "zzababoi", 2)]
    #[case("zzaabb", "zzabab", 1)]
    #[case("abcdef", "badcfe", 3)]
    #[case("klmb", "klm", 1)]
    #[case("klm", "klmb", 1)]
    #[case("test", "text", 1)]
    #[case("test", "tset", 1)]
    #[case("test", "qwy", 4)]
    #[case("test", "testit", 2)]
    #[case("test", "tesst", 1)]
    #[case("test", "tet", 1)]
    #[case("cat", "hat", 1)]
    #[case("Niall", "Neil", 3)]
    #[case("aluminum", "Catalan", 7)]
    #[case("ATCG", "TAGC", 2)]
    #[case("ab", "ba", 1)]
    #[case("ab", "cde", 3)]
    #[case("ab", "ac", 1)]
    #[case("ab", "bc", 2)]
    fn function_str(#[case] s1: &str, #[case] s2: &str, #[case] exp: usize) {
        let res1 = damerau_levenshtein(s1, s2);
        let res2 = damerau_levenshtein_restricted(s1, s2);
        assert!(res1 == res2);
        assert!(res1 == exp);
    }

    #[test]
    fn restricted() {
        let a = DamerauLevenshtein {
            restricted: true,
            ..Default::default()
        };
        assert!(a.for_str("ab", "bca").val() == 3);
        assert!(a.for_str("abcd", "bdac").val() == 4);
    }

    #[test]
    fn unrestricted() {
        let a = DamerauLevenshtein::default();
        assert!(a.for_str("ab", "bca").val() == 2);
        assert!(a.for_str("abcd", "bdac").val() == 3);
    }

    proptest! {
        #[test]
        fn prop_default(s1 in ".*", s2 in ".*") {
            let res = damerau_levenshtein(&s1, &s2);
            let res2 = damerau_levenshtein(&s2, &s1);
            prop_assert_eq!(res, res2);
            prop_assert!(res <= s1.len() || res <= s2.len());
        }

        #[test]
        fn prop_restricted(s1 in ".*", s2 in ".*") {
            let res = damerau_levenshtein_restricted(&s1, &s2);
            let res2 = damerau_levenshtein_restricted(&s2, &s1);
            prop_assert_eq!(res, res2);
            prop_assert!(res <= s1.len() || res <= s2.len());
        }
    }
}
