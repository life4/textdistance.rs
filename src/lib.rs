pub mod textdistance {
    mod algorithm;
    mod hamming;
    mod lcsseq;
    mod lcsstr;
    mod ratcliff_obershelp;

    pub use self::algorithm::Algorithm;
    pub use self::hamming::{hamming, Hamming};
    pub use self::lcsseq::{lcsseq, LCSSeq};
    pub use self::lcsstr::{lcsstr, LCSStr};
    pub use self::ratcliff_obershelp::{ratcliff_obershelp, RatcliffObershelp};
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::textdistance::Algorithm;
    use proptest::prelude::*;

    fn get_algs() -> Vec<Box<dyn Algorithm>> {
        vec![
            Box::new(textdistance::Hamming {}),
            Box::new(textdistance::LCSSeq {}),
            Box::new(textdistance::LCSStr {}),
            Box::new(textdistance::RatcliffObershelp {}),
        ]
    }

    #[test]
    fn basic() {
        for alg in get_algs() {
            assert_eq!(alg.distance("", ""), 0);
            assert!(alg.distance("ab", "cde") > 0);
            assert!(alg.similarity("spam", "qwer") == 0);
            assert_eq!(alg.normalized_distance("", ""), 0.);
            assert_eq!(alg.normalized_similarity("", ""), 1.);
        }
    }

    proptest! {
        #[test]
        fn prop(s1 in ".*", s2 in ".*") {
            for alg in get_algs() {
                let d = alg.distance(&s1, &s2);
                let s = alg.similarity(&s1, &s2);

                let nd = alg.normalized_distance(&s1, &s2);
                assert!(nd >= 0.);
                assert!(nd <= 1.);

                let ns = alg.normalized_similarity(&s1, &s2);
                assert!(ns >= 0.);
                assert!(ns <= 1.);

                assert!((ns + nd) > 0.9999999);
                assert!((ns + nd) < 1.0000001);

                if d < s {
                    assert!(nd < ns);
                } else if d > s {
                    assert!(nd > ns);
                } else {
                    assert!(nd == ns);
                }
            }
        }

        fn prop_same(s in ".*") {
            for alg in get_algs() {
                let nd = alg.normalized_distance(&s, &s);
                assert_eq!(nd, 0.);

                let ns = alg.normalized_similarity(&s, &s);
                assert_eq!(ns, 1.);
            }
        }
    }
}
