//! Tversky index
#![cfg(feature = "std")]
use crate::counter::Counter;
use crate::{Algorithm, Result};

/// [Tversky similarity] is a generalization of [`SorensenDice`] and [`Jaccard`].
///
/// [Tversky similarity]: https://en.wikipedia.org/wiki/Tversky_index
/// [`SorensenDice`]: crate::SorensenDice
/// [`Jaccard`]: crate::Jaccard
pub struct Tversky {
    /// α, the weight of the first sequence (the "prototype").
    pub alpha: f64,
    /// β, the weight of the second sequence (the "variant").
    pub beta: f64,
    /// The symmetric Tversky index bias parameter.
    pub bias: f64,
}

impl Default for Tversky {
    fn default() -> Self {
        Self {
            alpha: 1.,
            beta: 1.,
            bias: 0.,
        }
    }
}

impl Algorithm<f64> for Tversky {
    fn for_iter<C, E>(&self, s1: C, s2: C) -> Result<f64>
    where
        C: Iterator<Item = E>,
        E: Eq + core::hash::Hash,
    {
        let c1 = Counter::from_iter(s1);
        let c2 = Counter::from_iter(s2);
        let ic = c1.intersect_count(&c2);
        let n1 = c1.count();
        let n2 = c2.count();

        if n1 == 0 && n2 == 0 {
            return Result {
                abs: 1.,
                is_distance: false,
                max: 1.,
                len1: c1.count(),
                len2: c2.count(),
            };
        }

        let denom = self.alpha * (n1 - ic) as f64 + self.beta * (n2 - ic) as f64;
        let res = (ic as f64 + self.bias) / (ic as f64 + denom);
        Result {
            abs: res,
            is_distance: false,
            max: 1.,
            len1: c1.count(),
            len2: c2.count(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::str::{jaccard, sorensen_dice, tversky};
    use assert2::assert;
    use proptest::prelude::*;
    use rstest::rstest;

    fn is_close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1E-5
    }

    #[rstest]
    #[case("", "", 1.)]
    #[case("nelson", "", 0.)]
    #[case("", "neilsen", 0.)]
    fn function_str(#[case] s1: &str, #[case] s2: &str, #[case] exp: f64) {
        let act = tversky(s1, s2);
        let ok = is_close(act, exp);
        assert!(ok, "tversky({}, {}) is {}, not {}", s1, s2, act, exp);
    }

    proptest! {
        #[test]
        fn sorensen_dice_eqivalence(s1 in ".*", s2 in ".*") {
            let tv = Tversky{alpha: 0.5, beta: 0.5, ..Default::default()};
            let tv_res = tv.for_str(&s1, &s2);
            let sd_res = sorensen_dice(&s1, &s2);
            prop_assert!(is_close(tv_res.nval(), sd_res));
        }

        #[test]
        fn tanimoto_eqivalence(s1 in ".*", s2 in ".*") {
            let tv = Tversky{alpha: 1., beta: 1., ..Default::default()};
            let tv_res = tv.for_str(&s1, &s2);
            let sd_res = jaccard(&s1, &s2);
            prop_assert!(is_close(tv_res.nval(), sd_res));
        }
    }
}
