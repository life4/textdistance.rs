//! Levenshtein distance
use crate::{Algorithm, Result};
use alloc::vec::Vec;

/// [Levenshtein distance] is an edit distance between two sequences.
///
/// It is the minimum number of single-character edits (insertions, deletions or substitutions)
/// required to change one word into the other.
///
/// See also [`DamerauLevenshtein`](crate::DamerauLevenshtein) which is an extended
/// version of this algorithm that also includes transpositions.
///
/// [Levenshtein distance]: https://en.wikipedia.org/wiki/Levenshtein_distance
pub struct Levenshtein {
    /// The cost of removing a character.
    pub del_cost: usize,

    /// The cost of adding a new character.
    pub ins_cost: usize,

    /// The cost of replacing a character with another one.
    pub sub_cost: usize,
}

impl Default for Levenshtein {
    fn default() -> Self {
        Self {
            del_cost: 1,
            ins_cost: 1,
            sub_cost: 1,
        }
    }
}

impl Algorithm<usize> for Levenshtein {
    fn for_iter<C, E>(&self, s1: C, s2: C) -> Result<usize>
    where
        C: Iterator<Item = E>,
        E: Eq,
    {
        let s1: Vec<E> = s1.collect();
        let l1 = s1.len();
        if l1 == 0 {
            let l2 = s2.count();
            return Result {
                abs: l2,
                is_distance: true,
                max: l1.max(l2),
                len1: l1,
                len2: l2,
            };
        }

        let mut cache: Vec<usize> = (1..).take(l1).collect();
        let mut dist1;
        let mut dist2;

        let mut result = 0;
        let mut l2 = 0;
        for (i2, c2) in s2.enumerate() {
            result = i2;
            dist1 = i2;
            l2 += 1;

            for (i1, c1) in s1.iter().enumerate() {
                dist2 = if c1 == &c2 {
                    dist1
                } else {
                    dist1 + self.sub_cost
                };
                dist1 = cache[i1];
                result = if dist1 > result {
                    if dist2 > result {
                        result + self.del_cost
                    } else {
                        dist2
                    }
                } else if dist2 > dist1 {
                    dist1 + self.ins_cost
                } else {
                    dist2
                };
                cache[i1] = result;
            }
        }
        if l2 == 0 {
            return Result {
                abs: l1,
                is_distance: true,
                max: l1.max(l2),
                len1: l1,
                len2: l2,
            };
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
    use crate::str::levenshtein;
    use assert2::assert;
    use proptest::prelude::*;
    use rstest::rstest;

    #[rstest]
    #[case("", "", 0)]
    #[case("", "\0", 1)]
    #[case("", "abc", 3)]
    #[case("abc", "", 3)]
    #[case("sitting", "sitting", 0)]
    #[case("sitting", "kitten", 3)]
    #[case("example", "samples", 3)]
    #[case("distance", "difference", 5)]
    #[case("test", "text", 1)]
    #[case("test", "tset", 2)]
    #[case("test", "qwe", 4)]
    #[case("test", "testit", 2)]
    #[case("test", "tesst", 1)]
    #[case("test", "tet", 1)]
    // parity with levenshtein-rs
    #[case("sitting", "kitten", 3)]
    #[case("gumbo", "gambol", 2)]
    #[case("saturday", "sunday", 3)]
    #[case("a", "b", 1)]
    #[case("ab", "ac", 1)]
    #[case("ac", "bc", 1)]
    #[case("abc", "axc", 1)]
    #[case("xabxcdxxefxgx", "1ab2cd34ef5g6", 6)]
    #[case("xabxcdxxefxgx", "abcdefg", 6)]
    #[case("javawasneat", "scalaisgreat", 7)]
    #[case("example", "samples", 3)]
    #[case("sturgeon", "urgently", 6)]
    #[case("levenshtein", "frankenstein", 6)]
    #[case("distance", "difference", 5)]
    #[case("kitten", "sitting", 3)]
    #[case("Tier", "Tor", 2)]
    fn function_str(#[case] s1: &str, #[case] s2: &str, #[case] exp: usize) {
        assert!(levenshtein(s1, s2) == exp);
    }

    proptest! {
        #[test]
        fn prop(s1 in ".*", s2 in ".*") {
            let res = levenshtein(&s1, &s2);
            let res2 = levenshtein(&s2, &s1);
            prop_assert_eq!(res, res2);
            prop_assert!(res <= s1.len() || res <= s2.len());
        }
    }
}
