use std::collections::HashMap;

use super::algorithm::{Algorithm, Result};

pub struct DamerauLevenshtein {}
use std::hash::Hash;

impl Default for DamerauLevenshtein {
    fn default() -> Self {
        Self {}
    }
}

impl Algorithm for DamerauLevenshtein {
    fn for_iter<C, E>(&self, s1: C, s2: C) -> Result
    where
        C: Iterator<Item = E>,
        E: Eq + Hash,
    {
        let s1: Vec<E> = s1.collect();
        let s2: Vec<E> = s2.collect();
        let l1 = s1.len();
        let l2 = s2.len();
        let max_dist = l2 + l1;

        let mut mat: Vec<Vec<usize>> = vec![vec![0; l2 + 2]; l1 + 2];
        mat[0][0] = max_dist;
        for i in 0..(l1 + 1) {
            mat[i + 1][0] = max_dist;
            mat[i + 1][1] = i;
        }
        for i in 0..(l2 + 1) {
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

                let cost = if c1 == c2 { 0 } else { 1 };
                mat[i1 + 1][i2 + 1] = min4(
                    mat[i1 + 1][i2] + 1,                                 // deletion
                    mat[i1][i2 + 1] + 1,                                 // insertion
                    mat[i1][i2] + cost,                                  // substitution
                    mat[last][db] + (i1 - last - 1) + 1 + (i2 - db - 1), // transposition
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
}

fn min4(a: usize, b: usize, c: usize, d: usize) -> usize {
    a.min(b).min(c).min(d)
}

pub fn damerau_levenshtein(s1: &str, s2: &str) -> usize {
    let a: DamerauLevenshtein = Default::default();
    a.for_str(s1, s2).dist()
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn function() {
        let f = damerau_levenshtein;
        assert_eq!(f("", ""), 0);
        assert_eq!(f("", "\0"), 1);
        assert_eq!(f("", "abc"), 3);
        assert_eq!(f("abc", ""), 3);
        assert_eq!(f("hannah", "hannha"), 1);
        assert_eq!(f("FOO", "BOR"), 2);
        assert_eq!(f("BAR", "BOR"), 1);
        assert_eq!(f("hansi", "hasni"), 1);
        assert_eq!(f("zzaabbio", "zzababoi"), 2);
        assert_eq!(f("zzaabb", "zzabab"), 1);
        assert_eq!(f("abcdef", "badcfe"), 3);
        assert_eq!(f("klmb", "klm"), 1);
        assert_eq!(f("klm", "klmb"), 1);

        assert_eq!(f("test", "text"), 1);
        assert_eq!(f("test", "tset"), 1);
        assert_eq!(f("test", "qwy"), 4);
        assert_eq!(f("test", "testit"), 2);
        assert_eq!(f("test", "tesst"), 1);
        assert_eq!(f("test", "tet"), 1);

        assert_eq!(f("cat", "hat"), 1);
        assert_eq!(f("Niall", "Neil"), 3);
        assert_eq!(f("aluminum", "Catalan"), 7);
        assert_eq!(f("ATCG", "TAGC"), 2);

        assert_eq!(f("ab", "ba"), 1);
        assert_eq!(f("ab", "cde"), 3);
        assert_eq!(f("ab", "ac"), 1);
        assert_eq!(f("ab", "bc"), 2);
    }

    proptest! {
        #[test]
        fn prop(s1 in ".*", s2 in ".*") {
            let res = damerau_levenshtein(&s1, &s2);
            let res2 = damerau_levenshtein(&s2, &s1);
            prop_assert_eq!(res, res2);
            prop_assert!(res <= s1.len() || res <= s2.len());
        }
    }
}
