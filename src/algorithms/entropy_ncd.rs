//! Entropy-based Normalized Compression Distance
#![cfg(feature = "std")]
use crate::counter::Counter;
use crate::{Algorithm, Result};
use core::hash::Hash;

/// Entropy-based [Normalized Compression Distance].
///
/// It shows how different two inputs are based on their [Entropy].
///
/// [Normalized Compression Distance]: https://en.wikipedia.org/wiki/Normalized_compression_distance
/// [Entropy]: https://en.wikipedia.org/wiki/Entropy_(information_theory)
pub struct EntropyNCD {
    /// The base of logarithm for the entropy calculation. Default: 2.
    pub base: usize,

    /// A non-negative base value to add to entropy of all inputs,
    /// so that the entropy is never zero. It accounts for all real-world compression
    /// algorithms having a fixed header with metadata. Default: 1.
    pub correction: f64,
}

impl Default for EntropyNCD {
    fn default() -> Self {
        Self {
            base: 2,
            correction: 1.,
        }
    }
}

impl EntropyNCD {
    fn compress<E: Hash + Eq>(&self, c: &Counter<E>) -> f64 {
        debug_assert!(self.correction >= 0.);
        let total_count = c.count();
        let mut entropy = 0.0;
        for element_count in c.values() {
            let p = *element_count as f64 / total_count as f64;
            entropy -= p * p.log(self.base as f64);
        }
        debug_assert!(entropy >= 0.);
        self.correction + entropy
    }
}

impl Algorithm<f64> for EntropyNCD {
    fn for_iter<C, E>(&self, s1: C, s2: C) -> Result<f64>
    where
        C: Iterator<Item = E>,
        E: Eq + core::hash::Hash,
    {
        let c1 = Counter::from_iter(s1);
        let c2 = Counter::from_iter(s2);
        let cm = c1.merge(&c2);
        let cl1 = self.compress(&c1);
        let cl2 = self.compress(&c2);
        let res: f64 = if cl1 == 0. && cl2 == 0. {
            0.
        } else {
            let clt = self.compress(&cm);
            (clt - cl1.min(cl2)) / cl1.max(cl2)
        };
        Result {
            abs: res,
            is_distance: true,
            max: 1.,
            len1: c1.count(),
            len2: c2.count(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Counter;
    use super::EntropyNCD;
    use crate::str::entropy_ncd;
    use assert2::assert;
    use proptest::prelude::*;
    use rstest::rstest;

    fn is_close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1E-5
    }

    #[rstest]
    #[case("", "", 0.)]
    #[case("test", "test", 0.)]
    #[case("aaa", "bbb", 1.)]
    #[case("test", "nani", 0.4)]
    // parity with textdistance
    fn function_str(#[case] s1: &str, #[case] s2: &str, #[case] exp: f64) {
        let act = entropy_ncd(s1, s2);
        let ok = is_close(act, exp);
        assert!(ok, "entropy_ncd({}, {}) is {}, not {}", s1, s2, act, exp);
    }

    #[rstest]
    #[case("", 0.)]
    #[case("hhhh", 0.)]
    #[case("hi", 1.)]
    #[case("hii", 0.9182958340544896)]
    #[case("hhi", 0.9182958340544896)]
    #[case("test", 1.5)]
    #[case("nani", 1.5)]
    #[case("testnani", 2.5)]
    fn compress(#[case] s: &str, #[case] exp: f64) {
        let c = Counter::from_iter(s.chars());
        let alg = EntropyNCD {
            correction: 0.,
            ..Default::default()
        };
        let act = alg.compress(&c);
        let ok = is_close(act, exp);
        assert!(ok, "compress({}) is {}, not {}", s, act, exp);
    }

    proptest! {
        #[test]
        fn compress_idempotency(s in ".+") {
            let c = Counter::from_iter(s.chars());
            let e = EntropyNCD::default();
            let r1 = e.compress(&c);
            let r2 = e.compress(&c.merge(&c));
            prop_assert!(r2 < r1 * 2.);
        }
    }
}
