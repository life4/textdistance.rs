//! Sift4 distance
use crate::{Algorithm, Result};
use alloc::vec::Vec;
use core::num::Wrapping;

/// [Sift4 distance] is an edit algorithm designed to be "fast and relatively accurate".
///
/// The original blog post describes 3 different implementations of the algorithm,
/// this is the "common" one. The main difference from [`Sift4Simple`](crate::Sift4Simple)
/// is the support for `max_distance` that can be used to stop calculating the distance
/// after a certain threshold.
///
/// [Sift4 distance]: https://siderite.dev/blog/super-fast-and-accurate-string-distance.html
pub struct Sift4Common {
    /// The number of characters to search for matching letters. Default: 5.
    pub max_offset: usize,

    /// The distance at which the algorithm should stop computing the value
    /// and just exit (the strings are too different anyway). Default: 0.
    pub max_distance: usize,
}

impl Default for Sift4Common {
    fn default() -> Self {
        Self {
            max_distance: 0,
            max_offset: 5,
        }
    }
}

impl Algorithm<usize> for Sift4Common {
    fn for_vec<E: Eq>(&self, s1: &[E], s2: &[E]) -> Result<usize> {
        let l1 = s1.len();
        let l2 = s2.len();

        // if l1 == 0 {
        //     return l2;
        // }
        // if l2 == 0 {
        //     return l1;
        // }

        // NOTE: c1 and c2 are Wrapping because one step of the algorithm can temporarily underflow them, causing panics in debug builds
        let mut c1 = Wrapping(0); // cursor for string 1
        let mut c2 = Wrapping(0); // cursor for string 2
        let mut lcss = 0; // largest common subsequence
        let mut local_cs = 0; // local common substring
        let mut trans = 0; // number of transpositions ('ab' vs 'ba')
        let mut offset_arr: Vec<Offset> = Vec::new(); // offset pair array, for computing the transpositions
        while (c1.0 < l1) && (c2.0 < l2) {
            if s1[c1.0] == s2[c2.0] {
                local_cs += 1;
                let mut is_trans = false;
                //see if current match is a transposition
                let mut i = 0;
                while i < offset_arr.len() {
                    let ofs = &mut offset_arr[i];
                    if c1.0 <= ofs.c1 || c2.0 <= ofs.c2 {
                        // when two matches cross, the one considered a transposition is the one with the largest difference in offsets
                        is_trans = c1.0.abs_diff(c2.0) >= ofs.c1.abs_diff(ofs.c2);
                        if is_trans {
                            trans += 1;
                        } else if !ofs.trans {
                            ofs.trans = true;
                            trans += 1;
                        }
                        break;
                    } else if c1.0 > ofs.c2 && c2.0 > ofs.c1 {
                        offset_arr.remove(i);
                    } else {
                        i += 1;
                    }
                }
                offset_arr.push(Offset {
                    c1: c1.0,
                    c2: c2.0,
                    trans: is_trans,
                });
            } else {
                lcss += local_cs;
                local_cs = 0;
                if c1 != c2 {
                    let t = c1.min(c2); //using min allows the computation of transpositions
                    c1 = t;
                    c2 = t;
                }
                if self.max_distance != 0 {
                    let temporary_distance = c1.0.max(c2.0) - lcss + trans;
                    if temporary_distance > self.max_distance {
                        return Result {
                            abs: temporary_distance,
                            is_distance: true,
                            max: l1.max(l2),
                            len1: l1,
                            len2: l2,
                        };
                    }
                }
                //if matching characters are found, remove 1 from both cursors (they get incremented at the end of the loop)
                //so that we can have only one code block handling matches
                for i in 0..self.max_offset {
                    if c1.0 + i >= l1 && c2.0 + i >= l2 {
                        break;
                    }
                    if (c1.0 + i < l1) && (s1[c1.0 + i] == s2[c2.0]) {
                        c1 += i;
                        c1 -= 1; // NOTE: c1 may underflow here

                        c2 -= 1; // NOTE: c2 may underflow here

                        break;
                    }
                    if (c2.0 + i < l2) && (s1[c1.0] == s2[c2.0 + i]) {
                        c1 -= 1; // NOTE: c1 may underflow here

                        c2 += i;
                        c2 -= 1; // NOTE: c2 may underflow here

                        break;
                    }
                }
            }

            // NOTE: If c1 or c2 underflowed in the previous loop, this ensures that they return to 0
            c1 += 1;
            c2 += 1;

            // this covers the case where the last match is on the last token in list, so that it can compute transpositions correctly
            if (c1.0 >= l1) || (c2.0 >= l2) {
                lcss += local_cs;
                local_cs = 0;
                let t = c1.min(c2);
                c1 = t;
                c2 = t;
            }
        }
        Result {
            abs: l1.max(l2) - lcss - local_cs + trans,
            is_distance: true,
            max: l1.max(l2),
            len1: l1,
            len2: l2,
        }
    }
}

struct Offset {
    c1: usize,
    c2: usize,
    trans: bool,
}

#[cfg(test)]
mod tests {
    use crate::str::sift4_common;
    use assert2::assert;
    use rstest::rstest;

    #[rstest]
    // parity with another Rust implementation
    #[case("London", "Lond", 2)]
    #[case("Chicago", "Chiag", 2)]
    // #[case("Los Angeles", "Angeles", 4)]
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
    // Underflow panic regression tests
    #[case("aaaaaa |", "baaaaa", 3)]
    #[case("/", "®/", 1)]
    fn function_str(#[case] s1: &str, #[case] s2: &str, #[case] exp: usize) {
        assert!(sift4_common(s1, s2) == exp);
    }
}
