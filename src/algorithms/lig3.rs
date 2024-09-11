//! LIG3 similarity
use super::hamming::Hamming;
use super::levenshtein::Levenshtein;
use crate::{Algorithm, Result};
use core::hash::Hash;

/// [LIG3 similarity] is a normalization of [`Hamming`] by [`Levenshtein`].
///
/// [LIG3 similarity]: https://github.com/chrislit/abydos/blob/master/abydos/distance/_lig3.py
pub struct LIG3 {
    /// Algorithm instance to use for calculating Levenshtein distance.
    pub levenshtein: Levenshtein,

    /// Algorithm instance to use for calculating Hamming similarity.
    pub hamming: Hamming,
}

impl Default for LIG3 {
    fn default() -> Self {
        Self {
            levenshtein: Levenshtein::default(),
            #[allow(clippy::needless_update)]
            hamming: Hamming {
                truncate: false,
                ..Default::default()
            },
        }
    }
}

impl Algorithm<f64> for LIG3 {
    fn for_vec<E>(&self, s1: &[E], s2: &[E]) -> Result<f64>
    where
        E: Eq + Hash,
    {
        let lev_res = self.levenshtein.for_vec(s1, s2);
        let lev = lev_res.dist();
        let ham = self.hamming.for_vec(s1, s2).sim();
        let res = if lev == 0 && ham == 0 {
            1.
        } else {
            (2 * ham) as f64 / (2 * ham + lev) as f64
        };
        Result {
            abs: res,
            is_distance: false,
            max: 1.0,
            len1: lev_res.len1,
            len2: lev_res.len2,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::str::lig3;
    use assert2::assert;
    use rstest::rstest;

    fn is_close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1E-5
    }

    #[rstest]
    // parity with abydos
    #[case("cat", "hat", 0.8)]
    #[case("Niall", "Neil", 0.5714285714285714)]
    #[case("aluminum", "Catalan", 0.0)]
    #[case("ATCG", "TAGC", 0.0)]
    #[case("Glavin", "Glawyn", 0.8)]
    #[case("Williams", "Vylliems", 0.7692307692307693)]
    #[case("Lewis", "Louis", 0.75)]
    #[case("Alex", "Alexander", 0.6153846153846154)]
    #[case("Wild", "Wildsmith", 0.6153846153846154)]
    #[case("Bram", "Bramberley", 0.5714285714285714)]
    fn function_str(#[case] s1: &str, #[case] s2: &str, #[case] exp: f64) {
        let act = lig3(s1, s2);
        let ok = is_close(act, exp);
        assert!(ok, "lig3({}, {}) is {}, not {}", s1, s2, act, exp);
    }
}
