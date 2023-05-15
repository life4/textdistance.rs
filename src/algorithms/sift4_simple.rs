//! Sift4 distance
use crate::{Algorithm, Result};

/// [Sift4 distance] is an edit algorithm designed to be "fast and relatively accurate".
///
/// The original blog post describes 3 different implementations of the algorithm,
/// this is the "simplest" one.
///
/// [Sift4 distance]: https://siderite.dev/blog/super-fast-and-accurate-string-distance.html
pub struct Sift4Simple {
    /// The number of characters to search for matching letters.
    pub max_offset: usize,
    // max_distance: usize,
}

impl Default for Sift4Simple {
    fn default() -> Self {
        Self {
            // max_distance: 0,
            max_offset: 5,
        }
    }
}

impl Algorithm<usize> for Sift4Simple {
    fn for_vec<E: Eq>(&self, s1: &[E], s2: &[E]) -> Result<usize> {
        let l1 = s1.len();
        let l2 = s2.len();

        let mut c1 = 0; // cursor for string 1
        let mut c2 = 0; // cursor for string 2
        let mut lcss = 0; // largest common subsequence
        let mut local_cs = 0; // local common substring

        while c1 < l1 && c2 < l2 {
            if s1[c1] == s2[c2] {
                local_cs += 1;
            } else {
                lcss += local_cs;
                local_cs = 0;
                if c1 != c2 {
                    c1 = c1.min(c2);
                    c2 = c1; // using min allows the computation of transpositions
                }

                for i in 0..self.max_offset {
                    if !(c1 + 1 < l1 || c2 + i < l2) {
                        break;
                    }

                    if c1 + i < l1 && s1[c1 + i] == s2[c2] {
                        c1 += i;
                        local_cs += 1;
                        break;
                    }
                    if (c2 + i < l2) && (s1[c1] == s2[c2 + i]) {
                        c2 += i;
                        local_cs += 1;
                        break;
                    }
                }
            }
            c1 += 1;
            c2 += 1;
        }
        Result {
            abs: l1.max(l2) - lcss - local_cs,
            is_distance: true,
            max: l1.max(l2),
            len1: l1,
            len2: l2,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::str::sift4_simple;
    use assert2::assert;
    use rstest::rstest;

    #[rstest]
    // parity with another Rust implementation
    #[case("London", "Lond", 2)]
    #[case("Chicago", "Chiag", 2)]
    #[case("Los Angeles", "Angeles", 4)]
    #[case("Bangkok", "Bagrok", 2)]
    #[case("San Francisco", "san Francisco", 1)]
    #[case("New York", "new York", 1)]
    #[case("San Francisco", "", 13)]
    #[case("", "New York", 8)]
    // parity with Swift implementation
    #[case("a", "a", 0)]
    #[case("a", "b", 1)]
    #[case("aa", "aabb", 2)]
    #[case("aaaa", "aabb", 2)]
    #[case("abba", "aabb", 1)]
    #[case("aaaa", "abbb", 3)]
    #[case("123 nowhere ave", "123 n0where 4ve", 2)]
    #[case("bisectable6", "disectable6", 1)]
    fn function_str(#[case] s1: &str, #[case] s2: &str, #[case] exp: usize) {
        assert!(sift4_simple(s1, s2) == exp);
    }
}
