//! Cosine similarity
#![cfg(feature = "std")]
use crate::counter::Counter;
use crate::{Algorithm, Result};

/// [Cosine similarity] is the cosine of the angle between two vectors.
///
/// This is how many symbols the given strings have in common
/// divided by the square root of the product of the strings' lengths.
///
/// [Cosine similarity]: https://en.wikipedia.org/wiki/Cosine_similarity
#[derive(Default)]
pub struct Cosine {}

impl Algorithm<f64> for Cosine {
    fn for_iter<C, E>(&self, s1: C, s2: C) -> Result<f64>
    where
        C: Iterator<Item = E>,
        E: Eq + core::hash::Hash,
    {
        let c1 = Counter::from_iter(s1);
        let c2 = Counter::from_iter(s2);
        let n1 = c1.count();
        let n2 = c2.count();
        let res = match (n1, n2) {
            (0, 0) => 1.,
            (_, 0) | (0, _) => 0.,
            (_, _) => {
                let ic = c1.intersect_count(&c2);
                ic as f64 / ((n1 * n2) as f64).sqrt()
            }
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
    use crate::str::cosine;
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
    #[case("test", "text", 3. / 4.)]
    #[case("nelson", "neilsen", 0.771516)]
    fn function_str(#[case] s1: &str, #[case] s2: &str, #[case] exp: f64) {
        let act = cosine(s1, s2);
        let ok = is_close(act, exp);
        assert!(ok, "cosine({}, {}) is {}, not {}", s1, s2, act, exp);
    }
}
