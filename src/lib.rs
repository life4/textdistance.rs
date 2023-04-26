pub mod textdistance {
    pub mod str;

    mod algorithm;
    mod damerau_levenshtein;
    mod hamming;
    mod jaro;
    mod lcsseq;
    mod lcsstr;
    mod levenshtein;
    mod ratcliff_obershelp;
    mod sift4;

    pub use self::algorithm::{Algorithm, Result};
    pub use self::damerau_levenshtein::DamerauLevenshtein;
    pub use self::hamming::Hamming;
    pub use self::jaro::Jaro;
    pub use self::lcsseq::LCSSeq;
    pub use self::lcsstr::LCSStr;
    pub use self::levenshtein::Levenshtein;
    pub use self::ratcliff_obershelp::RatcliffObershelp;
    pub use self::sift4::Sift4;
}

#[cfg(test)]
mod tests {
    use crate::textdistance::*;
    use assert2::assert;
    use proptest::prelude::*;
    use rstest::rstest;

    const ALGS: usize = 7;

    fn get_result(alg: usize, s1: &str, s2: &str) -> Result<usize> {
        match alg {
            1 => Hamming::default().for_str(s1, s2),
            2 => LCSSeq::default().for_str(s1, s2),
            3 => LCSStr::default().for_str(s1, s2),
            4 => RatcliffObershelp::default().for_str(s1, s2),
            5 => Levenshtein::default().for_str(s1, s2),
            6 => DamerauLevenshtein::default().for_str(s1, s2),
            7 => Sift4::default().for_str(s1, s2),
            _ => panic!("there are not so many algorithms!"),
        }
    }

    #[rstest]
    #[case::hamming(1)]
    #[case::lcsseq(2)]
    #[case::lcsstr(3)]
    #[case::ratcliff_obershelp(4)]
    #[case::levenshtein(5)]
    #[case::damerau_levenshtein(6)]
    #[case::sift4(7)]
    fn basic(#[case] alg: usize) {
        assert!(get_result(alg, "", "").dist() == 0);
        assert!(get_result(alg, "ab", "cde").dist() > 0);
        assert!(get_result(alg, "spam", "qwer").sim() == 0);
        assert!(get_result(alg, "", "").ndist() == 0.);
        assert!(get_result(alg, "", "").nsim() == 1.);
    }

    fn is_close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1E-9
    }

    proptest! {
        #[test]
        fn prop(s1 in ".*", s2 in ".*") {
            for alg in 1..ALGS {
                let res = get_result(alg, &s1, &s2);
                let d = res.dist();
                let s = res.sim();

                let nd = res.ndist();
                prop_assert!(nd.is_finite());
                prop_assert!(nd >= 0.);
                prop_assert!(nd <= 1.);

                let ns = res.nsim();
                prop_assert!(ns.is_finite());
                prop_assert!(ns >= 0.);
                prop_assert!(ns <= 1.);

                prop_assert!(is_close(ns + nd, 1.), "{} + {} == 1", nd, ns);

                if d < s {
                    prop_assert!(nd < ns, "{} < {}", nd, ns);
                } else if d > s {
                    prop_assert!(nd > ns, "{} > {}", nd, ns);
                } else if !s1.is_empty() && !s2.is_empty() {
                    prop_assert!(nd == ns, "{} == {}", nd, ns);
                }
                prop_assert!(res.abs == d || res.abs == s);

                prop_assert_eq!(res.len1, s1.chars().count());
                prop_assert_eq!(res.len2, s2.chars().count());
                prop_assert!(res.max >= res.len1.min(res.len2));
            }
        }

        #[test]
        fn prop_same(s in ".*") {
            for alg in 1..ALGS {
                let res = get_result(alg, &s, &s);
                let nd = res.ndist();
                prop_assert_eq!(nd, 0., "{} == 0.0", nd);
                let ns = res.nsim();
                prop_assert_eq!(ns, 1., "{} == 1.0", ns);
            }
        }
    }
}
