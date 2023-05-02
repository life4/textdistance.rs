use std::hash::Hash;

/// A base trait for all distance/similarity algorithms.
pub trait Algorithm<R> {
    /// Calculate distance/similarity for iterators.
    fn for_iter<C, E>(&self, s1: C, s2: C) -> Result<R>
    where
        C: Iterator<Item = E>,
        E: Eq + Copy + Hash,
    {
        let s1: Vec<E> = s1.collect();
        let s2: Vec<E> = s2.collect();
        self.for_vec(&s1, &s2)
    }

    /// Calculate distance/similarity for vectors.
    fn for_vec<E>(&self, s1: &[E], s2: &[E]) -> Result<R>
    where
        E: Eq + Copy + Hash,
    {
        self.for_iter(s1.iter(), s2.iter())
    }

    /// Calculate distance/similarity for strings.
    fn for_str(&self, s1: &str, s2: &str) -> Result<R> {
        self.for_iter(s1.chars(), s2.chars())
    }

    /// Calculate distance/similarity for words in strings.
    fn for_words(&self, s1: &str, s2: &str) -> Result<R> {
        self.for_iter(s1.split_whitespace(), s2.split_whitespace())
    }

    /// Calculate distance/similarity for bigrams in strings.
    fn for_bigrams(&self, s1: &str, s2: &str) -> Result<R> {
        self.for_iter(bigrams(s1), bigrams(s2))
    }
}

fn bigrams(s: &str) -> impl Iterator<Item = (char, char)> + '_ {
    s.chars().zip(s.chars().skip(1))
}

/// Result of a distance/similarity algorithm.
pub struct Result<R> {
    pub is_distance: bool,
    pub abs: R,
    pub max: R,
    pub len1: usize,
    pub len2: usize,
}

impl Result<usize> {
    /// Raw value of the metric.
    ///
    /// It is equivalent to `dist` for distance metrics
    /// and to `sim` for similarity metrics.
    pub fn val(&self) -> usize {
        self.abs
    }

    /// Absolute distance.
    ///
    /// A non-negative number showing how different the two sequences are.
    /// Two exactly the same sequences have the distance 0.
    ///
    /// The highest possible number varies based on the length of the input strings.
    /// Most often, each increment of this value indicates one symbol that differs
    /// in the input sequences.
    pub fn dist(&self) -> usize {
        if self.is_distance {
            self.abs
        } else {
            self.max - self.abs
        }
    }

    /// Absolute similarity.
    ///
    /// A non-negative number showing how similar the two sequences are.
    /// Two absolutely different sequences have the similarity 0.
    ///
    /// The highest possible number varies based on the length of the input strings.
    /// Most often, each increment of this value indicates one symbol that is the same
    /// in both sequences.
    pub fn sim(&self) -> usize {
        if self.is_distance {
            self.max - self.abs
        } else {
            self.abs
        }
    }

    /// Normalized raw value of the metric.
    ///
    /// It is equivalent to `ndist` for distance metrics
    /// and to `nsim` for similarity metrics.
    pub fn nval(&self) -> f64 {
        if self.is_distance {
            self.ndist()
        } else {
            self.nsim()
        }
    }

    /// Normalized distance.
    ///
    /// A number from 0.0 to 1.0 showing how different the two sequences are.
    /// 0.0 indicates that the sequences are the same,
    /// and 1.0 indicates that the sequences are very different.
    pub fn ndist(&self) -> f64 {
        if self.max == 0 {
            self.dist() as f64
        } else {
            self.dist() as f64 / self.max as f64
        }
    }

    /// Normalized similarity.
    ///
    /// A number from 0.0 to 1.0 showing how similar the two sequences are.
    /// 0.0 indicates that the sequences are very different,
    /// and 1.0 indicates that the sequences are the same.
    pub fn nsim(&self) -> f64 {
        if self.max == 0 {
            1.0
        } else {
            self.sim() as f64 / self.max as f64
        }
    }
}

impl Result<f64> {
    /// Normalized raw value of the metric.
    ///
    /// It is equivalent to `ndist` for distance metrics
    /// and to `nsim` for similarity metrics.
    pub fn nval(&self) -> f64 {
        self.abs
    }

    /// Normalized distance.
    ///
    /// A number from 0.0 to 1.0 showing how different the two sequences are.
    /// 0.0 indicates that the sequences are the same,
    /// and 1.0 indicates that the sequences are very different.
    pub fn ndist(&self) -> f64 {
        if self.is_distance {
            self.abs
        } else {
            self.max - self.abs
        }
    }

    /// Normalized similarity.
    ///
    /// A number from 0.0 to 1.0 showing how similar the two sequences are.
    /// 0.0 indicates that the sequences are very different,
    /// and 1.0 indicates that the sequences are the same.
    pub fn nsim(&self) -> f64 {
        if self.is_distance {
            self.max - self.abs
        } else {
            self.abs
        }
    }
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
