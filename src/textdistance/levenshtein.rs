use super::algorithm::{Algorithm, Result};

pub struct Levenshtein {}

impl Default for Levenshtein {
    fn default() -> Self {
        Self {}
    }
}

impl Algorithm for Levenshtein {
    fn for_iter<C, E>(&self, s1: C, s2: C) -> Result
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

pub fn levenshtein(s1: &str, s2: &str) -> usize {
    let a: Levenshtein = Default::default();
    a.for_str(s1, s2).dist()
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn function() {
        let f = levenshtein;
        assert_eq!(f("", ""), 0);
        assert_eq!(f("", "\0"), 1);
        assert_eq!(f("", "abc"), 3);
        assert_eq!(f("abc", ""), 3);
        assert_eq!(f("sitting", "sitting"), 0);
        assert_eq!(f("sitting", "kitten"), 3);
        assert_eq!(f("example", "samples"), 3);
        assert_eq!(f("distance", "difference"), 5);
        assert_eq!(f("test", "text"), 1);
        assert_eq!(f("test", "tset"), 2);
        assert_eq!(f("test", "qwe"), 4);
        assert_eq!(f("test", "testit"), 2);
        assert_eq!(f("test", "tesst"), 1);
        assert_eq!(f("test", "tet"), 1);
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
