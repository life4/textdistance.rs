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
    pub fn nval(&self) -> f64 {
        self.abs
    }
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
