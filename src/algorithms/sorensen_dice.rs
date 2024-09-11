//! Sørensen-Dice coefficient
#![cfg(feature = "std")]
use crate::counter::Counter;
use crate::{Algorithm, Result};

/// [Sørensen–Dice similarity] is a ratio of common chars to total chars in the given strings.
///
/// [Sørensen–Dice similarity]: https://en.wikipedia.org/wiki/S%C3%B8rensen%E2%80%93Dice_coefficient
#[derive(Default)]
pub struct SorensenDice {}

impl Algorithm<f64> for SorensenDice {
    fn for_iter<C, E>(&self, s1: C, s2: C) -> Result<f64>
    where
        C: Iterator<Item = E>,
        E: Eq + core::hash::Hash,
    {
        let c1 = Counter::from_iter(s1);
        let c2 = Counter::from_iter(s2);
        let cn = c1.count() + c2.count();
        let res = if cn == 0 {
            1.
        } else {
            let ic = c1.intersect_count(&c2);
            (2 * ic) as f64 / cn as f64
        };
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
    use crate::str::sorensen_dice;
    use crate::{Algorithm, SorensenDice};
    use assert2::assert;
    use rstest::rstest;

    fn is_close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1E-5
    }

    #[rstest]
    #[case("", "", 1.)]
    #[case("nelson", "", 0.)]
    #[case("", "neilsen", 0.)]
    // parity with textdistance
    #[case("test", "text", 2.0 * 3. / 8.)]
    fn function_str(#[case] s1: &str, #[case] s2: &str, #[case] exp: f64) {
        let act = sorensen_dice(s1, s2);
        let ok = is_close(act, exp);
        assert!(ok, "sorensen_dice({}, {}) is {}, not {}", s1, s2, act, exp);
    }

    #[rstest]
    // parity with strsim
    #[case("a", "a", 1.0)]
    #[case("", "", 1.0)]
    #[case("apple event", "apple event", 1.0)]
    #[case("iphone", "iphone x", 0.9090909090909091)]
    #[case("french", "quebec", 0.0)]
    #[case("france", "france", 1.0)]
    #[case("fRaNce", "france", 0.2)]
    #[case("healed", "sealed", 0.8)]
    #[case("web applications", "applications of the web", 0.7878787878)]
    #[case("this has one extra word", "this has one word", 0.7741935483870968)]
    #[case(
        "this will have a typo somewhere",
        "this will huve a typo somewhere",
        0.92
    )]
    #[case(
        "Olive-green table for sale, in extremely good condition.",
        "For sale: table in very good  condition, olive green in colour.",
        0.6060606060606061
    )]
    #[case(
        "Olive-green table for sale, in extremely good condition.",
        "For sale: green Subaru Impreza, 210,000 miles",
        0.2558139534883721
    )]
    #[case(
        "Olive-green table for sale, in extremely good condition.",
        "Wanted: mountain bike with at least 21 gears.",
        0.1411764705882353
    )]
    fn for_bigrams(#[case] s1: &str, #[case] s2: &str, #[case] exp: f64) {
        let s1 = &s1.replace(' ', "");
        let s2 = &s2.replace(' ', "");
        let act = SorensenDice::default().for_bigrams(s1, s2).nval();
        let ok = is_close(act, exp);
        assert!(ok, "sorensen_dice({}, {}) is {}, not {}", s1, s2, act, exp);
    }
}
