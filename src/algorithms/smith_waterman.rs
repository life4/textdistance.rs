//! Smith-Waterman sequence alignment
use crate::{Algorithm, Result};
use alloc::vec;
use alloc::vec::Vec;

/// [Smith-Waterman similarity] is edit-based and designed for nucleic acid (and protein) sequences.
///
/// [Smith-Waterman similarity]: https://en.wikipedia.org/wiki/Smith%E2%80%93Waterman_algorithm
pub struct SmithWaterman {
    /// The cost of an alignment gap. Default: 1.
    pub gap_cost: isize,

    /// The cost of symbols matching. Default: -1.
    pub match_cost: isize,

    /// The cost of symbols not matching. Default: 0.
    pub mismatch_cost: isize,
}

impl Default for SmithWaterman {
    fn default() -> Self {
        Self {
            gap_cost: 1,
            match_cost: -1,
            mismatch_cost: 0,
        }
    }
}

impl Algorithm<usize> for SmithWaterman {
    fn for_vec<E: Eq>(&self, s1: &[E], s2: &[E]) -> Result<usize> {
        let l1 = s1.len();
        let l2 = s2.len();
        let mut dist_mat: Vec<Vec<isize>> = vec![vec![0; l2 + 1]; l1 + 1];
        for (i, sc1) in s1.iter().enumerate() {
            for (j, sc2) in s2.iter().enumerate() {
                let cost = if sc1 == sc2 {
                    self.match_cost
                } else {
                    self.mismatch_cost
                };
                let match_ = dist_mat[i][j] - cost;
                let delete = dist_mat[i][j + 1] - self.gap_cost;
                let insert = dist_mat[i + 1][j] - self.gap_cost;
                dist_mat[i + 1][j + 1] = 0.max(match_).max(delete).max(insert);
            }
        }
        let result = dist_mat[l1][l2];
        Result {
            #[allow(clippy::cast_sign_loss)]
            abs: result as usize,
            is_distance: false,
            max: l1.max(l2),
            len1: l1,
            len2: l2,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::str::smith_waterman;
    use assert2::assert;
    use rstest::rstest;

    #[rstest]
    // parity with textdistance
    #[case("hello", "world", 1)]
    #[case("abcd", "abce", 3)]
    #[case("AGACTAGTTAC", "CGAGACGT", 3)]
    #[case("qwe", "rty", 0)]
    #[case("qwe", "rty", 0)]
    #[case("check", "shrek", 2)]
    // parity with abydos
    #[case("cat", "hat", 2)]
    #[case("Niall", "Neil", 1)]
    #[case("aluminum", "Catalan", 0)]
    #[case("ATCG", "TAGC", 1)]
    fn function_str(#[case] s1: &str, #[case] s2: &str, #[case] exp: usize) {
        assert!(smith_waterman(s1, s2) == exp);
    }
}
