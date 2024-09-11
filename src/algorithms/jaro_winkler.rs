//! Jaro-Winkler similarity
use super::jaro::Jaro;
use crate::{Algorithm, Result};

/// [Jaro-Winkler similarity] is a variation of [`Jaro`] with a better rating for strings with a matching prefix.
///
/// The metric is always normalized on the interval from 0.0 to 1.0.
///
/// [Jaro-Winkler similarity]: https://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance
pub struct JaroWinkler {
    /// The Jaro instance to use to calculate the classic Jaro similarity.
    pub jaro: Jaro,

    /// `p` is a scailing factor for how much Jaro score is adjusted
    /// for the common prefix. The default is 0.1, must not be higher than
    /// `1/ℓ` where ℓ is the `max_prefix` value (4 by default).
    pub prefix_weight: f64,

    /// `ℓ` is the maximum length of the common prefix. The default is 4.
    pub max_prefix: usize,
}

impl Default for JaroWinkler {
    fn default() -> Self {
        Self {
            jaro: Jaro::default(),
            prefix_weight: 0.1,
            max_prefix: 4,
        }
    }
}

impl JaroWinkler {
    fn winklerize<C, E>(&self, jaro: f64, s1: C, s2: C) -> f64
    where
        C: Iterator<Item = E>,
        E: Eq + core::hash::Hash,
    {
        debug_assert!(self.prefix_weight * self.max_prefix as f64 <= 1.0);
        let mut prefix_len = 0;
        for (e1, e2) in s1.zip(s2) {
            if e1 == e2 {
                prefix_len += 1;
                if prefix_len == self.max_prefix {
                    break;
                }
            } else {
                break;
            }
        }
        jaro + (self.prefix_weight * prefix_len as f64 * (1.0 - jaro))
    }
}

impl Algorithm<f64> for JaroWinkler {
    fn for_vec<E>(&self, s1: &[E], s2: &[E]) -> Result<f64>
    where
        E: Eq + core::hash::Hash,
    {
        let jaro = self.jaro.for_vec(s1, s2).nval();
        Result {
            abs: self.winklerize(jaro, s1.iter(), s2.iter()),
            is_distance: false,
            max: 1.0,
            len1: s1.len(),
            len2: s2.len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::str::jaro_winkler;
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
    #[case("testabctest", "testöঙ香test", 0.890909)]
    #[case("testöঙ香test", "testabctest", 0.890909)]
    #[case("dixon", "dicksonx", 0.8133333)]
    #[case("dwayne", "duane", 0.8400000)]
    #[case("martha", "marhta", 0.9611111)]
    #[case("Friedrich Nietzsche", "Fran-Paul Sartre", 0.561988)]
    #[case("Thorkel", "Thorgier", 0.867857)]
    #[case("Dinsdale", "D", 0.737500)]
    // These fail because strsim doesn't limit the max prefix length:
    // #[case("cheeseburger", "cheese fries", 0.911111)]
    // #[case("thequickbrownfoxjumpedoverx", "thequickbrownfoxjumpedovery", 1.0)]

    // parity with jellyfish
    #[case("dixon", "dicksonx", 0.81333333)]
    #[case("martha", "marhta", 0.961111111)]
    #[case("dwayne", "duane", 0.84)]
    #[case("William", "Williams", 0.975)]
    #[case("", "foo", 0.)]
    #[case("a", "a", 1.)]
    #[case("abc", "xyz", 0.)]
    #[case("aaaa", "aaaaa", 0.96)]
    #[case("orangutan-kumquat", "orangutan kumquat", 0.97647058)]
    #[case("jaz", "jal", 0.8222222)]
    #[case("@", "@@", 0.85)]
    #[case("0", "0@", 0.85)]
    #[case("a", "ab", 0.85)]
    #[case("012345", "0123456", 0.9714285)]
    #[case("012abc", "012abcd", 0.9714285)]
    #[case("012abc", "013abcd", 0.879365079)]
    #[case("a1bc", "a1be", 0.8833333)]
    fn function_str(#[case] s1: &str, #[case] s2: &str, #[case] exp: f64) {
        let act = jaro_winkler(s1, s2);
        let ok = is_close(act, exp);
        assert!(ok, "jaro_winkler({}, {}) is {}, not {}", s1, s2, act, exp);
    }
}
