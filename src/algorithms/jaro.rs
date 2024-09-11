//! Jaro similarity
use crate::{Algorithm, Result};
use alloc::vec;

/// [Jaro similarity] is calculated based on the number of transpositions to turn one string into the other.
///
/// The metric is always normalized on the interval from 0.0 to 1.0.
///
/// See also [`JaroWinkler`](crate::JaroWinkler).
///
/// [Jaro similarity]: https://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance#Jaro_similarity
#[derive(Default)]
pub struct Jaro {}

impl Algorithm<f64> for Jaro {
    fn for_vec<E: Eq>(&self, s1: &[E], s2: &[E]) -> Result<f64> {
        let l1 = s1.len();
        let l2 = s2.len();

        if l1 == 0 || l2 == 0 {
            let result = if l1 == 0 && l2 == 0 { 1. } else { 0. };
            return Result {
                abs: result,
                is_distance: false,
                max: 1.0,
                len1: l1,
                len2: l2,
            };
        }
        if l1 == 1 && l2 == 1 {
            let result = if s1[0] == s2[0] { 1.0 } else { 0.0 };
            return Result {
                abs: result,
                is_distance: false,
                max: 1.0,
                len1: l1,
                len2: l2,
            };
        }

        let search_range = l1.max(l2) / 2 - 1;

        let mut s2_consumed = vec![false; l2];
        let mut matches: usize = 0;

        let mut n_trans = 0.;
        let mut b_match_index = 0;

        for (i, a_elem) in s1.iter().enumerate() {
            let min_bound =
            // prevent integer wrapping
            if i > search_range {
                i - search_range
            } else {
                0
            };

            let max_bound = usize::min(l2 - 1, i + search_range);

            if min_bound > max_bound {
                continue;
            }

            for (j, b_elem) in s2.iter().enumerate() {
                if min_bound <= j && j <= max_bound && a_elem == b_elem && !s2_consumed[j] {
                    s2_consumed[j] = true;
                    matches += 1;

                    if j < b_match_index {
                        n_trans += 1.;
                    }
                    b_match_index = j;

                    break;
                }
            }
        }

        let result = if matches == 0 {
            0.
        } else {
            let ms = matches as f64;
            ((ms / l1 as f64) + (ms / l2 as f64) + ((ms - n_trans) / ms)) / 3.
        };

        Result {
            abs: result,
            is_distance: false,
            max: 1.,
            len1: l1,
            len2: l2,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::str::jaro;
    use assert2::assert;
    use rstest::rstest;

    fn is_close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1E-5
    }

    #[rstest]
    // parity with strsim-rs
    #[case("", "", 1.)]
    #[case("a", "a", 1.)]
    #[case("Jaro-Winkler", "Jaro-Winkler", 1.)]
    #[case("", "jaro-winkler", 0.)]
    #[case("distance", "", 0.)]
    #[case("a", "b", 0.)]
    #[case("dixon", "dicksonx", 0.76667)]
    #[case("a", "ab", 0.83333)]
    #[case("ab", "a", 0.83333)]
    #[case("dwayne", "duane", 0.82222)]
    #[case("Friedrich Nietzsche", "Jean-Paul Sartre", 0.39189)]
    fn function_str(#[case] s1: &str, #[case] s2: &str, #[case] exp: f64) {
        let act = jaro(s1, s2);
        let ok = is_close(act, exp);
        assert!(ok, "jaro({}, {}) is {}, not {}", s1, s2, act, exp);
    }
}
