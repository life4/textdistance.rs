use super::algorithm::{Algorithm, Result};

#[derive(Default)]
pub struct Levenshtein {}

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
                dist2 = if c1 == &c2 { dist1 } else { dist1 + 1 };
                dist1 = cache[i1];
                result = if dist1 > result {
                    if dist2 > result {
                        result + 1
                    } else {
                        dist2
                    }
                } else if dist2 > dist1 {
                    dist1 + 1
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
    use crate::textdistance::str::levenshtein;
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
    fn function_str(#[case] s1: &str, #[case] s2: &str, #[case] exp: usize) {
        assert_eq!(levenshtein(s1, s2), exp);
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
