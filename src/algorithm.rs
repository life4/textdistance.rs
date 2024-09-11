use super::Result;
use alloc::vec::Vec;
use core::hash::Hash;

/// A base trait for all distance/similarity algorithms.
///
///     use textdistance::{Algorithm, Hamming};
///     let h = Hamming::default();
///     let res = h.for_str("abc", "acbd");
///     assert!(res.val() == 3);
///
pub trait Algorithm<R> {
    /// Calculate distance/similarity for iterators.
    ///
    ///     use textdistance::{Algorithm, Hamming};
    ///     let h = Hamming::default();
    ///     let res = h.for_iter(1..4, 1..6);
    ///     assert!(res.val() == 2);
    ///
    fn for_iter<C, E>(&self, s1: C, s2: C) -> Result<R>
    where
        C: Iterator<Item = E>,
        E: Eq + Hash,
    {
        let s1: Vec<E> = s1.collect();
        let s2: Vec<E> = s2.collect();
        self.for_vec(&s1, &s2)
    }

    /// Calculate distance/similarity for vectors.
    ///
    ///     use textdistance::{Algorithm, Hamming};
    ///     let h = Hamming::default();
    ///     let res = h.for_vec(&vec![1, 2, 3], &vec![1, 3, 2, 4]);
    ///     assert!(res.val() == 3);
    ///
    fn for_vec<E>(&self, s1: &[E], s2: &[E]) -> Result<R>
    where
        E: Eq + Hash,
    {
        self.for_iter(s1.iter(), s2.iter())
    }

    /// Calculate distance/similarity for strings.
    ///
    ///     use textdistance::{Algorithm, Hamming};
    ///     let h = Hamming::default();
    ///     let res = h.for_str("abc", "acbd");
    ///     assert!(res.val() == 3);
    ///
    fn for_str(&self, s1: &str, s2: &str) -> Result<R> {
        self.for_iter(s1.chars(), s2.chars())
    }

    /// Calculate distance/similarity for words in strings.
    ///
    ///     use textdistance::{Algorithm, Hamming};
    ///     let h = Hamming::default();
    ///     let res = h.for_words("the first edition", "the second edition");
    ///     assert!(res.val() == 1);
    ///
    fn for_words(&self, s1: &str, s2: &str) -> Result<R> {
        self.for_iter(s1.split_whitespace(), s2.split_whitespace())
    }

    /// Calculate distance/similarity for bigrams in strings.
    ///
    ///     use textdistance::{Algorithm, Hamming};
    ///     let h = Hamming::default();
    ///     let res = h.for_str("abd", "abcd");
    ///     assert!(res.val() == 2); // 3 bigrams (ab, bc, cd), only "ab" matches
    ///
    fn for_bigrams(&self, s1: &str, s2: &str) -> Result<R> {
        self.for_iter(bigrams(s1), bigrams(s2))
    }
}

fn bigrams(s: &str) -> impl Iterator<Item = (char, char)> + '_ {
    s.chars().zip(s.chars().skip(1))
}

#[cfg(test)]
mod tests {
    use super::Algorithm;
    use crate::Hamming;
    use assert2::assert;
    // use proptest::prelude::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![], vec![], 0)]
    #[case(vec![1], vec![1], 0)]
    #[case(vec![1], vec![5], 1)]
    #[case(vec![3], vec![5], 1)]
    #[case(vec![3, 4, 5, 6], vec![1, 4, 5, 6, 7], 2)]
    fn for_vec(#[case] s1: Vec<usize>, #[case] s2: Vec<usize>, #[case] exp: usize) {
        let h = Hamming::default();
        assert!(h.for_vec(&s1, &s2).val() == exp);
    }

    #[rstest]
    #[case("", "", 0)]
    #[case("", "\0", 1)]
    #[case("", "abc", 3)]
    #[case("abc", "", 3)]
    #[case("sitting", "sitting", 0)]
    #[case("abcdefg", "hijklmn", 7)]
    #[case("karolin", "kathrin", 3)]
    #[case("hello", "world", 4)]
    fn for_str(#[case] s1: &str, #[case] s2: &str, #[case] exp: usize) {
        let h = Hamming::default();
        assert!(h.for_str(s1, s2).val() == exp);
    }

    #[rstest]
    #[case("", "", 0)]
    #[case("", "\0", 1)]
    #[case("", "abc", 1)]
    #[case("abc", "", 1)]
    #[case("oh hi mark", "oh hi world", 1)]
    #[case("oh hi mark", "oh hi mad world", 2)]
    #[case("oh hi mark", "greeting you mad world", 4)]
    fn for_words(#[case] s1: &str, #[case] s2: &str, #[case] exp: usize) {
        let h = Hamming::default();
        assert!(h.for_words(s1, s2).val() == exp);
    }

    #[rstest]
    #[case("", "", 0)]
    // #[case("", "a", 1)]
    #[case("", "abc", 2)]
    #[case("abc", "", 2)]
    #[case("oh hi mark", "oh ho mark", 2)]
    fn for_bigrams(#[case] s1: &str, #[case] s2: &str, #[case] exp: usize) {
        let h = Hamming::default();
        assert!(h.for_bigrams(s1, s2).val() == exp);
    }
}
