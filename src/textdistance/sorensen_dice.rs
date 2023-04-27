use super::algorithm::{Algorithm, Result};
use super::counter::Counter;

#[derive(Default)]
pub struct SorensenDice {}

impl Algorithm<f64> for SorensenDice {
    fn for_iter<C, E>(&self, s1: C, s2: C) -> Result<f64>
    where
        C: Iterator<Item = E>,
        E: Eq + Copy + std::hash::Hash,
    {
        let c1 = Counter::from_iter(s1);
        let c2 = Counter::from_iter(s2);
        let ic = c1.intersect_count(&c2);
        let cn = c1.count() + c2.count();
        let res = if cn == 0 {
            1.
        } else {
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
    use crate::textdistance::str::sorensen_dice;
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
}
