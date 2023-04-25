use super::algorithm::{Algorithm, Result};

/// [Sift4] is a distance algorithm designed to be "fast and relatively accurate".
///
/// [Sift4]: https://siderite.dev/blog/super-fast-and-accurate-string-distance.html
pub struct Sift4 {
    simple: bool,
    // max_distance: usize,
    max_offset: usize,
}

impl Default for Sift4 {
    fn default() -> Self {
        Self {
            simple: true,
            // max_distance: 0,
            max_offset: 5,
        }
    }
}

impl Sift4 {
    fn get_simple<E: Eq>(&self, s1: &[E], s2: &[E]) -> Result<usize> {
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

impl Algorithm<usize> for Sift4 {
    fn for_vec<E>(&self, s1: &[E], s2: &[E]) -> Result<usize>
    where
        E: Eq + Copy + std::hash::Hash,
    {
        assert!(self.simple);
        self.get_simple(s1, s2)
    }
}

#[cfg(test)]
mod tests {
    use crate::textdistance::str::sift4;

    #[test]
    fn function() {
        let f = sift4;

        // parity with another Rust implementation
        assert_eq!(f("London", "Lond"), 2);
        assert_eq!(f("Chicago", "Chiag"), 2);
        assert_eq!(f("Los Angeles", "Angeles"), 4);
        assert_eq!(f("Bangkok", "Bagrok"), 2);
        assert_eq!(f("San Francisco", "san Francisco"), 1);
        assert_eq!(f("New York", "new York"), 1);
        assert_eq!(f("San Francisco", ""), 13);
        assert_eq!(f("", "New York"), 8);

        // parity with Swift implementation
        assert_eq!(f("a", "a"), 0);
        assert_eq!(f("a", "b"), 1);
        assert_eq!(f("aa", "aabb"), 2);
        assert_eq!(f("aaaa", "aabb"), 2);
        assert_eq!(f("abba", "aabb"), 1);
        assert_eq!(f("aaaa", "abbb"), 3);
        assert_eq!(f("123 nowhere ave", "123 n0where 4ve"), 2);
        assert_eq!(f("bisectable6", "disectable6"), 1);
    }
}
