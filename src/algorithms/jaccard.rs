use super::algorithm::{Algorithm, Result};
use super::counter::Counter;

#[derive(Default)]
pub struct Jaccard {}

impl Algorithm<f64> for Jaccard {
    fn for_iter<C, E>(&self, s1: C, s2: C) -> Result<f64>
    where
        C: Iterator<Item = E>,
        E: Eq + Copy + std::hash::Hash,
    {
        let c1 = Counter::from_iter(s1);
        let c2 = Counter::from_iter(s2);
        let uc = c1.union_count(&c2);
        let res = if uc == 0 {
            1.
        } else {
            let ic = c1.intersect_count(&c2);
            ic as f64 / uc as f64
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
    use crate::str::jaccard;
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
    #[case("nelson", "neilsen", 5. / 8.)]
    #[case("test", "text", 3. / 5.)]
    #[case("decide", "resize", 3. / 9.)]
    fn function_str(#[case] s1: &str, #[case] s2: &str, #[case] exp: f64) {
        let act = jaccard(s1, s2);
        let ok = is_close(act, exp);
        assert!(ok, "jaccard({}, {}) is {}, not {}", s1, s2, act, exp);
    }
}
