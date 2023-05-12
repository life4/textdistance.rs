//! Entropy-based Normalized Compression Distance
use crate::algorithm::{Algorithm, Result};
use crate::counter::Counter;
use std::collections::HashMap;
use std::hash::Hash;

/// [Arithmetic coding]-based [Normalized Compression Distance].
///
/// It shows how different two inputs are based on their [Arithmetic coding] compressed size.
///
/// [Arithmetic coding]: https://en.wikipedia.org/wiki/Arithmetic_coding
/// [Normalized Compression Distance]: https://en.wikipedia.org/wiki/Normalized_compression_distance
#[derive(Default)]
pub struct ArithNCD {}

impl ArithNCD {
    fn compress<C, E>(&self, s: C, counts: &Counter<E>) -> usize
    where
        C: Iterator<Item = E>,
        E: Hash + Eq + Copy,
    {
        let (minval, maxval) = self.get_range(s, counts);

        let mut delta: f64 = (maxval - minval) / 2.;
        let mut nbits: usize = 0;
        while delta < 1. {
            nbits += 1;
            delta *= 2.;
        }
        debug_assert!(nbits != 0);
        nbits
    }

    fn get_range<C, E>(&self, s: C, counts: &Counter<E>) -> (f64, f64)
    where
        C: Iterator<Item = E>,
        E: Hash + Eq + Copy,
    {
        let total_letters = counts.count();

        // calculate probability pairs
        let mut prob_pairs: HashMap<&E, (f64, f64)> = HashMap::new();
        let mut cumulative_count: usize = 0;
        let sorted = counts.most_common();
        for (char, current_count) in sorted {
            let v1 = cumulative_count as f64 / total_letters as f64;
            let v2 = current_count as f64 / total_letters as f64;
            prob_pairs.insert(char, (v1, v2));
            cumulative_count += current_count;
        }
        debug_assert_eq!(cumulative_count, total_letters);

        // calculate the range
        let mut start: f64 = 0.;
        let mut width: f64 = 1.;
        for char in s {
            let (prob_start, prob_width) = prob_pairs.get(&char).unwrap();
            start += prob_start * width;
            width *= prob_width;
        }
        (start, (start + width))
    }
}

impl Algorithm<f64> for ArithNCD {
    fn for_vec<E>(&self, s1: &[E], s2: &[E]) -> Result<f64>
    where
        E: Eq + Copy + Hash,
    {
        let c1 = Counter::from_iter(s1);
        let c2 = Counter::from_iter(s2);
        let cm = c1.merge(&c2);
        let cl1 = self.compress(s1.iter(), &c1);
        let cl2 = self.compress(s2.iter(), &c2);
        let res: f64 = if cl1 == 0 && cl2 == 0 {
            0.
        } else {
            let clt = self.compress(s1.iter().chain(s2), &cm);
            (clt - cl1.min(cl2)) as f64 / cl1.max(cl2) as f64
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
    use super::ArithNCD;
    use super::Counter;
    use crate::str::arith_ncd;
    use assert2::assert;
    // use proptest::prelude::*;
    use rstest::rstest;

    fn is_close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1E-5
    }

    #[rstest]
    #[case("", "", 0.)]
    // #[case("test", "test", 0.)]
    // #[case("aaa", "bbb", 1.)]
    // parity with textdistance
    fn function_str(#[case] s1: &str, #[case] s2: &str, #[case] exp: f64) {
        let act = arith_ncd(s1, s2);
        let ok = is_close(act, exp);
        assert!(ok, "arith_ncd({}, {}) is {}, not {}", s1, s2, act, exp);
    }

    #[rstest]
    // parity with textdistance
    #[case::empty("", 0., 1.)]
    #[case::a("a", 0., 1.)]
    #[case::aa("aa", 0., 1.)]
    #[case::ab("ab", 1./4., 1./2.)]
    #[case::abb("abb", 2./3., 22./27.)]
    #[case::aab("aab", 8./27., 4./9.)]
    // #[case::abc("abc", 5./27., 2./9.)]
    // #[case::abcc("abcc", 11./16., 45./64.)]
    fn range(#[case] s: &str, #[case] exp1: f64, #[case] exp2: f64) {
        let c = Counter::from_iter(s.chars());
        let alg = ArithNCD::default();
        let (act1, act2) = alg.get_range(s.chars(), &c);
        let ok1 = is_close(act1, exp1);
        assert!(ok1, "get_range({})[0] is {}, not {}", s, act1, exp1);
        let ok2 = is_close(act2, exp2);
        assert!(ok2, "get_range({})[1] is {}, not {}", s, act2, exp2);
    }

    // proptest! {
    //     #[test]
    //     fn compress_idempotency(s in ".+") {
    //         let c = Counter::from_iter(s.chars());
    //         let e = ArithNCD::default();
    //         let r1 = e.compress(&c);
    //         let r2 = e.compress(&c.merge(&c));
    //         prop_assert!(r2 < r1 * 2);
    //     }
    // }
}
