pub trait Algorithm {
    fn from_iter<C, E>(&self, s1: C, s2: C) -> Result
    where
        C: Iterator<Item = E>,
        E: Eq + Copy;

    fn from_str(&self, s1: &str, s2: &str) -> Result {
        self.from_iter(s1.chars(), s2.chars())
    }

    /// Absolute similarity.
    ///
    /// A shortcut for `self.from_iter(s1, s2).sim()`.
    fn sim<C, E>(&self, s1: C, s2: C) -> usize
    where
        C: Iterator<Item = E>,
        E: Eq + Copy,
    {
        self.from_iter(s1, s2).sim()
    }

    /// Absolute similarity for strings.
    ///
    /// A shortcut for `self.from_str(s1, s2).sim()`.
    fn ssim<C, E>(&self, s1: &str, s2: &str) -> usize {
        self.from_str(s1, s2).sim()
    }

    /// Absolute distance.
    ///
    /// A shortcut for `self.from_iter(s1, s2).dist()`.
    fn dist<C, E>(&self, s1: C, s2: C) -> usize
    where
        C: Iterator<Item = E>,
        E: Eq + Copy,
    {
        self.from_iter(s1, s2).dist()
    }

    /// Normalized similarity.
    ///
    /// A shortcut for `self.from_iter(s1, s2).nsim()`.
    fn nsim<C, E>(&self, s1: C, s2: C) -> f64
    where
        C: Iterator<Item = E>,
        E: Eq + Copy,
    {
        self.from_iter(s1, s2).nsim()
    }

    /// Normalized distance.
    ///
    /// A shortcut for `self.from_iter(s1, s2).ndist()`.
    fn ndist<C, E>(&self, s1: C, s2: C) -> f64
    where
        C: Iterator<Item = E>,
        E: Eq + Copy,
    {
        self.from_iter(s1, s2).ndist()
    }
}

/// Result of a distance/similarity algorithm.
pub struct Result {
    pub is_distance: bool,
    pub abs: usize,
    pub max: usize,
    pub len1: usize,
    pub len2: usize,
}

impl Result {
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
