//! Roberts similarity
#![cfg(feature = "std")]
use crate::counter::Counter;
use crate::{Algorithm, Result};

/// [Roberts similarity].
///
/// The metric is always normalized on the interval from 0.0 to 1.0.
///
/// [Roberts similarity]: https://github.com/chrislit/abydos/blob/master/abydos/distance/_roberts.py
#[derive(Default)]
pub struct Roberts {}

impl Algorithm<f64> for Roberts {
    fn for_iter<C, E>(&self, s1: C, s2: C) -> Result<f64>
    where
        C: Iterator<Item = E>,
        E: Eq + core::hash::Hash,
    {
        let c1 = Counter::from_iter(s1);
        let c2 = Counter::from_iter(s2);
        let n1 = c1.count();
        let n2 = c2.count();
        if n1 == 0 && n2 == 0 {
            return Result {
                abs: 1.0,
                is_distance: false,
                max: 1.,
                len1: n1,
                len2: n2,
            };
        }

        let cm = c1.merge(&c2);
        let alphabet = cm.keys();
        let mut s1: f64 = 0.;
        let mut s2: usize = 0;
        for key in alphabet {
            let v1 = c1.get(key).unwrap_or(&0);
            let v2 = c2.get(key).unwrap_or(&0);
            if v1 != &0 && v2 != &0 {
                s1 += ((v1 + v2) * v1.min(v2)) as f64 / *v1.max(v2) as f64;
            }
            s2 += v1 + v2;
        }

        Result {
            abs: s1 / s2 as f64,
            is_distance: false,
            max: 1.,
            len1: n1,
            len2: n2,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::str::roberts;
    use assert2::assert;
    use rstest::rstest;

    fn is_close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1E-5
    }

    #[rstest]
    #[case("", "", 1.)]
    #[case("a", "a", 1.)]
    #[case("", "a", 0.)]
    #[case("a", "", 0.)]
    // Parity with abydos.
    // By default, abydos uses bi-grams with word separators to tokenize any passed text
    // for Roberts. And that's what gets tested. However, textdistance uses bag of chars
    // by default and doesn't add any word separators ever. So, instead of using results
    // from tests, I've put results of running the values through `Roberts(qval=1).sim(a, b)`.
    #[case("cat", "hat", 0.6666666666666666)]
    #[case("Niall", "Neil", 0.6111111111111112)]
    #[case("aluminum", "Catalan", 0.3555555555555555)]
    #[case("ATCG", "TAGC", 1.0)]
    #[case("Nigel", "Niall", 0.55)]
    #[case("Niall", "Nigel", 0.55)]
    #[case("Colin", "Coiln", 1.0)]
    #[case("Coiln", "Colin", 1.0)]
    #[case("ATCAACGAGT", "AACGATTAG", 0.9210526315789473)]
    fn function_str(#[case] s1: &str, #[case] s2: &str, #[case] exp: f64) {
        let act = roberts(s1, s2);
        let ok = is_close(act, exp);
        assert!(ok, "roberts({}, {}) is {}, not {}", s1, s2, act, exp);
    }
}
