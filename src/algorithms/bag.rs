//! Bag distance
#![cfg(feature = "std")]
use crate::counter::Counter;
use crate::{Algorithm, Result};

/// [Bag distance] is how many max items there are in one sequence that aren't in the other.
///
/// [Bag distance]: http://www-db.disi.unibo.it/research/papers/SPIRE02.pdf
#[derive(Default)]
pub struct Bag {}

impl Algorithm<usize> for Bag {
    fn for_iter<C, E>(&self, s1: C, s2: C) -> Result<usize>
    where
        C: Iterator<Item = E>,
        E: Eq + core::hash::Hash,
    {
        let c1 = Counter::from_iter(s1);
        let c2 = Counter::from_iter(s2);
        let d1 = c1.diff_count(&c2);
        let d2 = c2.diff_count(&c1);
        let l1 = c1.count();
        let l2 = c2.count();

        Result {
            abs: d1.max(d2),
            is_distance: true,
            max: l1.max(l2),
            len1: l1,
            len2: l2,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::str::bag;
    use assert2::assert;
    use rstest::rstest;

    #[rstest]
    #[case("", "", 0)]
    // parity with textdistance
    #[case("qwe", "qwe", 0)]
    #[case("qwe", "erty", 3)]
    #[case("qwe", "ewq", 0)]
    #[case("qwe", "rtys", 4)]
    // parity with talisman
    #[case("cat", "hat", 1)]
    #[case("Niall", "Neil", 2)]
    #[case("aluminum", "Catalan", 5)]
    #[case("ATCG", "TAGC", 0)]
    fn function_str(#[case] s1: &str, #[case] s2: &str, #[case] exp: usize) {
        let act = bag(s1, s2);
        assert!(act == exp);
    }
}
