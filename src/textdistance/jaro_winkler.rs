use super::algorithm::{Algorithm, Result};
use super::jaro::Jaro;

#[derive(Default)]
pub struct JaroWinkler {
    jaro: Jaro,
}

impl Algorithm<f64> for JaroWinkler {
    fn for_vec<E>(&self, s1: &[E], s2: &[E]) -> Result<f64>
    where
        E: Eq + Copy + std::hash::Hash,
    {
        let mut prefix_len = 0;
        for (e1, e2) in s1.iter().zip(s2.iter()) {
            if e1 == e2 {
                prefix_len += 1;
            } else {
                break;
            }
        }

        let jaro_res = self.jaro.for_vec(s1, s2);
        let sim = jaro_res.abs + (0.1 * prefix_len as f64 * (1.0 - jaro_res.abs));
        Result {
            abs: sim.min(1.0),
            is_distance: false,
            max: 1.0,
            len1: jaro_res.len1,
            len2: jaro_res.len2,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::textdistance::str::jaro_winkler;
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
    #[case("cheeseburger", "cheese fries", 0.911111)]
    #[case("Thorkel", "Thorgier", 0.867857)]
    #[case("Dinsdale", "D", 0.737500)]
    #[case("thequickbrownfoxjumpedoverx", "thequickbrownfoxjumpedovery", 1.0)]
    fn function_str(#[case] s1: &str, #[case] s2: &str, #[case] exp: f64) {
        let act = jaro_winkler(s1, s2);
        let ok = is_close(act, exp);
        assert!(ok, "jaro_winkler({}, {}) is {}, not {}", s1, s2, act, exp);
    }
}
