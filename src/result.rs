/// Result of a distance/similarity algorithm.
pub struct Result<R> {
    /// Indicates if it is a distance or a similarity metric.
    pub(crate) is_distance: bool,

    /// Absolute raw value of the metric.
    pub(crate) abs: R,

    /// Maximum possible value for the input of the given length.
    pub(crate) max: R,

    /// Length of the first analyzed sequence.
    pub(crate) len1: usize,

    /// Length of the second analyzed sequence.
    pub(crate) len2: usize,
}

impl Result<usize> {
    /// Raw value of the metric.
    ///
    /// It is equivalent to `dist` for distance metrics
    /// and to `sim` for similarity metrics.
    ///
    ///     use textdistance::{Algorithm, Hamming};
    ///     let h = Hamming::default();
    ///     let res = h.for_str("abc", "acbd");
    ///     assert!(res.val() == 3);
    ///
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
    ///
    ///     use textdistance::{Algorithm, Hamming};
    ///     let h = Hamming::default();
    ///     let res = h.for_str("abc", "acbd");
    ///     assert!(res.dist() == 3);
    ///
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
    ///
    ///     use textdistance::{Algorithm, Hamming};
    ///     let h = Hamming::default();
    ///     let res = h.for_str("abc", "acbd");
    ///     assert!(res.sim() == 1); // "a"
    ///
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
    ///
    ///     use textdistance::{Algorithm, Hamming};
    ///     let h = Hamming::default();
    ///     let res = h.for_str("abc", "acbd");
    ///     assert!(res.nval() == 3.0 / 4.0);
    ///
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
    ///
    ///     use textdistance::{Algorithm, Hamming};
    ///     let h = Hamming::default();
    ///     let res = h.for_str("abc", "acbd");
    ///     assert!(res.ndist() == 3.0 / 4.0);
    ///
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
    ///
    ///     use textdistance::{Algorithm, Hamming};
    ///     let h = Hamming::default();
    ///     let res = h.for_str("abc", "acbd");
    ///     assert!(res.nsim() == 1.0 / 4.0);
    ///
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
    ///
    ///     use textdistance::{Algorithm, Jaro};
    ///     let h = Jaro::default();
    ///     let res = h.for_str("test", "tset");
    ///     assert_eq!(res.nval(), 0.9166666666666666);
    ///
    pub fn nval(&self) -> f64 {
        self.abs
    }

    /// Normalized distance.
    ///
    /// A number from 0.0 to 1.0 showing how different the two sequences are.
    /// 0.0 indicates that the sequences are the same,
    /// and 1.0 indicates that the sequences are very different.
    ///
    ///     use textdistance::{Algorithm, Jaro};
    ///     let h = Jaro::default();
    ///     let res = h.for_str("test", "tset");
    ///     assert_eq!(res.ndist(), 0.08333333333333337);
    ///
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
    ///
    ///     use textdistance::{Algorithm, Jaro};
    ///     let h = Jaro::default();
    ///     let res = h.for_str("test", "tset");
    ///     assert_eq!(res.nsim(), 0.9166666666666666);
    ///
    pub fn nsim(&self) -> f64 {
        if self.is_distance {
            self.max - self.abs
        } else {
            self.abs
        }
    }
}
