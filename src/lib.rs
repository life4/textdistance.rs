#![cfg_attr(not(any(feature = "std", test)), no_std)]
#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::cast_precision_loss,
    clippy::must_use_candidate,
    clippy::similar_names,
    clippy::unreadable_literal,
    clippy::doc_markdown,
    clippy::wildcard_imports
)]

extern crate alloc;

mod algorithm;
mod counter;
mod result;

pub mod nstr;
pub mod str;

mod algorithms {
    pub mod bag;
    pub mod cosine;
    pub mod damerau_levenshtein;
    pub mod entropy_ncd;
    pub mod hamming;
    pub mod jaccard;
    pub mod jaro;
    pub mod jaro_winkler;
    pub mod lcsseq;
    pub mod lcsstr;
    pub mod length;
    pub mod levenshtein;
    pub mod lig3;
    pub mod mlipns;
    pub mod overlap;
    pub mod prefix;
    pub mod ratcliff_obershelp;
    pub mod roberts;
    pub mod sift4_common;
    pub mod sift4_simple;
    pub mod smith_waterman;
    pub mod sorensen_dice;
    pub mod suffix;
    pub mod tversky;
    pub mod yujian_bo;
}

pub use self::algorithm::Algorithm;
#[cfg(feature = "std")]
pub use self::algorithms::bag::Bag;
#[cfg(feature = "std")]
pub use self::algorithms::cosine::Cosine;
#[cfg(feature = "std")]
pub use self::algorithms::damerau_levenshtein::DamerauLevenshtein;
#[cfg(feature = "std")]
pub use self::algorithms::entropy_ncd::EntropyNCD;
pub use self::algorithms::hamming::Hamming;
#[cfg(feature = "std")]
pub use self::algorithms::jaccard::Jaccard;
pub use self::algorithms::jaro::Jaro;
pub use self::algorithms::jaro_winkler::JaroWinkler;
pub use self::algorithms::lcsseq::LCSSeq;
pub use self::algorithms::lcsstr::LCSStr;
pub use self::algorithms::length::Length;
pub use self::algorithms::levenshtein::Levenshtein;
pub use self::algorithms::lig3::LIG3;
pub use self::algorithms::mlipns::MLIPNS;
#[cfg(feature = "std")]
pub use self::algorithms::overlap::Overlap;
pub use self::algorithms::prefix::Prefix;
pub use self::algorithms::ratcliff_obershelp::RatcliffObershelp;
#[cfg(feature = "std")]
pub use self::algorithms::roberts::Roberts;
pub use self::algorithms::sift4_common::Sift4Common;
pub use self::algorithms::sift4_simple::Sift4Simple;
pub use self::algorithms::smith_waterman::SmithWaterman;
#[cfg(feature = "std")]
pub use self::algorithms::sorensen_dice::SorensenDice;
pub use self::algorithms::suffix::Suffix;
#[cfg(feature = "std")]
pub use self::algorithms::tversky::Tversky;
pub use self::algorithms::yujian_bo::YujianBo;
pub use self::result::Result;

#[cfg(test)]
mod tests {
    #![allow(clippy::float_cmp)]

    use super::*;
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
            #[cfg(feature = "std")]
            6 => DamerauLevenshtein::default().for_str(s1, s2),
            7 => Sift4Simple::default().for_str(s1, s2),
            8 => MLIPNS::default().for_str(s1, s2),
            9 => Prefix::default().for_str(s1, s2),
            10 => Suffix::default().for_str(s1, s2),
            11 => Length::default().for_str(s1, s2),
            12 => Bag::default().for_str(s1, s2),
            13 => SmithWaterman::default().for_str(s1, s2),
            14 => Sift4Common::default().for_str(s1, s2),
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
            9 => EntropyNCD::default().for_str(s1, s2),
            10 => LIG3::default().for_str(s1, s2),
            11 => Roberts::default().for_str(s1, s2),
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
    #[case::sift4_simple(7)]
    #[case::mlipns(8)]
    #[case::prefix(9)]
    #[case::suffix(10)]
    #[case::length(11)]
    #[case::bag(12)]
    #[case::smith_waterman(13)]
    #[case::sift4_common(14)]
    fn basic_usize(#[case] alg: usize) {
        let empty_res = get_result(alg, "", "");
        assert!(empty_res.dist() == 0);
        if alg != 8 {
            assert!(get_result(alg, "ab", "cde").dist() > 0);
            assert!(get_result(alg, "ab", "cde").ndist() > 0.);
        }
        if alg != 11 {
            assert!(get_result(alg, "spam", "qwer").sim() == 0);
            assert!(get_result(alg, "spam", "qwer").nsim() == 0.);
        }
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
    #[case::entropy_ncd(9)]
    #[case::lig3(10)]
    #[case::roberts(11)]
    fn basic_f64(#[case] alg: usize) {
        let empty_res = get_result_f64(alg, "", "");
        assert!(get_result_f64(alg, "ab", "cde").ndist() > 0.);
        if alg != 3 && alg != 9 {
            assert!(get_result_f64(alg, "spam", "qwer").nsim() == 0.);
        }
        assert!(empty_res.ndist() == 0.);
        assert!(empty_res.nsim() == 1.);
        assert!(empty_res.max == 1.);
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
                prop_assert!(res.val() == d || res.val() == s);

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
                prop_assert_eq!(nd, 0., "{}: {} == 0.0", alg, nd);
                let ns = res.nsim();
                prop_assert_eq!(ns, 1., "{}: {} == 1.0", alg, ns);
            }
        }

        // strings should have lower distance if you add the same prefix to them
        fn prop_prefix(prefix in ".+", s1 in ".+", s2 in ".+") {
            for alg in 1..ALGS {
                let r1 = get_result(alg, &s1, &s2).ndist();
                let mut p1 = prefix.clone();
                let mut p2 = prefix.clone();
                p1.push_str(&s1);
                p2.push_str(&s2);
                let r2 = get_result(alg, &p1, &p2).ndist();
                prop_assert!(r1 > r2, "{}: {} > {}", alg, r1, r2);
            }
        }
    }
}
