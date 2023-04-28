pub mod textdistance {
    pub mod str;

    mod algorithm;
    mod cosine;
    mod counter;
    mod damerau_levenshtein;
    mod hamming;
    mod jaccard;
    mod jaro;
    mod jaro_winkler;
    mod lcsseq;
    mod lcsstr;
    mod levenshtein;
    mod mlipns;
    mod overlap;
    mod prefix;
    mod ratcliff_obershelp;
    mod sift4;
    mod sorensen_dice;
    mod suffix;
    mod tversky;
    mod yujian_bo;

    pub use self::algorithm::{Algorithm, Result};
    pub use self::cosine::Cosine;
    pub use self::damerau_levenshtein::DamerauLevenshtein;
    pub use self::hamming::Hamming;
    pub use self::jaccard::Jaccard;
    pub use self::jaro::Jaro;
    pub use self::jaro_winkler::JaroWinkler;
    pub use self::lcsseq::LCSSeq;
    pub use self::lcsstr::LCSStr;
    pub use self::levenshtein::Levenshtein;
    pub use self::mlipns::MLIPNS;
    pub use self::overlap::Overlap;
    pub use self::prefix::Prefix;
    pub use self::ratcliff_obershelp::RatcliffObershelp;
    pub use self::sift4::Sift4;
    pub use self::sorensen_dice::SorensenDice;
    pub use self::suffix::Suffix;
    pub use self::tversky::Tversky;
    pub use self::yujian_bo::YujianBo;
}

#[cfg(test)]
mod tests {
    use crate::textdistance::*;
    use assert2::assert;
    use proptest::prelude::*;
    use rstest::rstest;

    const ALGS: usize = 8;

    fn get_result(alg: usize, s1: &str, s2: &str) -> Result<usize> {
        match alg {
            1 => Hamming::default().for_str(s1, s2),
            2 => LCSSeq::default().for_str(s1, s2),
            3 => LCSStr::default().for_str(s1, s2),
            4 => RatcliffObershelp::default().for_str(s1, s2),
            5 => Levenshtein::default().for_str(s1, s2),
            6 => DamerauLevenshtein::default().for_str(s1, s2),
            7 => Sift4::default().for_str(s1, s2),
            8 => MLIPNS::default().for_str(s1, s2),
            9 => Prefix::default().for_str(s1, s2),
            10 => Suffix::default().for_str(s1, s2),
            _ => panic!("there are not so many algorithms!"),
        }
    }

    fn get_result_f64(alg: usize, s1: &str, s2: &str) -> Result<f64> {
        match alg {
            1 => Jaro::default().for_str(s1, s2),
            2 => JaroWinkler::default().for_str(s1, s2),
            3 => YujianBo::default().for_str(s1, s2),
            4 => Jaccard::default().for_str(s1, s2),
            5 => SorensenDice::default().for_str(s1, s2),
            6 => Tversky::default().for_str(s1, s2),
            7 => Overlap::default().for_str(s1, s2),
            8 => Cosine::default().for_str(s1, s2),
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
    #[case::mlipns(8)]
    #[case::prefix(8)]
    #[case::suffix(8)]
    fn basic_usize(#[case] alg: usize) {
        let empty_res = get_result(alg, "", "");
        assert!(empty_res.dist() == 0);
        if alg != 8 {
            assert!(get_result(alg, "ab", "cde").dist() > 0);
            assert!(get_result(alg, "ab", "cde").ndist() > 0.);
        }
        assert!(get_result(alg, "spam", "qwer").sim() == 0);
        assert!(get_result(alg, "spam", "qwer").nsim() == 0.);
        assert!(empty_res.ndist() == 0.);
        assert!(empty_res.nsim() == 1.);
    }

    #[rstest]
    #[case::jaro(1)]
    #[case::jaro_winkler(2)]
    #[case::yujian_bo(3)]
    #[case::jaccard(4)]
    #[case::sorensen_dice(5)]
    #[case::tversky(6)]
    #[case::overlap(7)]
    #[case::cosine(8)]
    fn basic_f64(#[case] alg: usize) {
        let empty_res = get_result_f64(alg, "", "");
        assert!(get_result_f64(alg, "ab", "cde").ndist() > 0.);
        if alg != 3 {
            assert!(get_result_f64(alg, "spam", "qwer").nsim() == 0.);
        }
        assert!(empty_res.ndist() == 0.);
        assert!(empty_res.nsim() == 1.);
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
