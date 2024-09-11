//! Helper functions providing the default normalized implementation of distance/similarity algorithms for strings.
//!
//! See also [`textdistance::str`](super::str) for non-normalized distance.
use super::*;

/// Calculate normalized unrestricted [Damerau-Levenshtein distance][1] for two strings.
///
/// A wrapper for [`DamerauLevenshtein`].
///
///     use textdistance::nstr::damerau_levenshtein;
///     assert!(damerau_levenshtein("abc", "acbd") == 2./4.); // "bc" swapped and "d" added
///
/// [1]: https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance
#[cfg(feature = "std")]
pub fn damerau_levenshtein(s1: &str, s2: &str) -> f64 {
    DamerauLevenshtein::default().for_str(s1, s2).nval()
}

/// Calculate normalized restricted [Damerau-Levenshtein distance][1] for two strings.
///
/// A wrapper for [`DamerauLevenshtein`].
///
///     use textdistance::nstr::damerau_levenshtein;
///     assert!(damerau_levenshtein("abc", "acbd") == 2./4.); // "bc" swapped and "d" added
///
/// [1]: https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance
#[cfg(feature = "std")]
pub fn damerau_levenshtein_restricted(s1: &str, s2: &str) -> f64 {
    let a = DamerauLevenshtein {
        restricted: true,
        ..Default::default()
    };
    a.for_str(s1, s2).nval()
}

/// Calculate normalized [Hamming distance][1] for two strings.
///
/// A wrapper for [`Hamming`].
///
///     use textdistance::nstr::hamming;
///     assert!(hamming("abc", "acbd") == 3./4.); // only "a" matches
///
/// [1]: https://en.wikipedia.org/wiki/Hamming_distance
pub fn hamming(s1: &str, s2: &str) -> f64 {
    Hamming::default().for_str(s1, s2).nval()
}

/// Calculate normalized the length of the [Longest Common SubSequence][1] for two strings.
///
/// A wrapper for [`LCSSeq`].
///
///     use textdistance::nstr::lcsseq;
///     assert!(lcsseq("abcdef", "xbcegf") == 4./6.); // "bcef"
///
/// [1]: https://en.wikipedia.org/wiki/Longest_common_subsequence
pub fn lcsseq(s1: &str, s2: &str) -> f64 {
    LCSSeq::default().for_str(s1, s2).nval()
}

/// Calculate normalized the length of the [Longest Common SubString][1] for two strings.
///
/// A wrapper for [`LCSStr`].
///
///     use textdistance::nstr::lcsstr;
///     assert!(lcsstr("abcdef", "xbcegf") == 2./6.); // "bc"
///
/// [1]: https://en.wikipedia.org/wiki/Longest_common_substring
pub fn lcsstr(s1: &str, s2: &str) -> f64 {
    LCSStr::default().for_str(s1, s2).nval()
}

/// Calculate normalized [Levenshtein distance][1] for two strings.
///
/// A wrapper for [`Levenshtein`].
///
///     use textdistance::nstr::levenshtein;
///     assert!(levenshtein("abc", "acbd") == 2./4.); // add "c" at 2 and then swap "c" with "d" at 4
///
/// [1]: https://en.wikipedia.org/wiki/Levenshtein_distance
pub fn levenshtein(s1: &str, s2: &str) -> f64 {
    Levenshtein::default().for_str(s1, s2).nval()
}

/// Calculate normalized [Ratcliff-Obershelp normalized similarity][1] for two strings.
///
/// A wrapper for [`RatcliffObershelp`].
///
///     use textdistance::nstr::ratcliff_obershelp;
///     assert_eq!(ratcliff_obershelp("abc", "acbd"), 0.5714285714285714);
///
/// [1]: https://en.wikipedia.org/wiki/Gestalt_pattern_matching
pub fn ratcliff_obershelp(s1: &str, s2: &str) -> f64 {
    RatcliffObershelp::default().for_str(s1, s2).nval()
}

/// Calculate normalized [Sift4 distance][1] for two strings using the "simplest" algorithm.
///
/// A wrapper for [`Sift4Simple`].
///
///     use textdistance::nstr::sift4_simple;
///     assert!(sift4_simple("abc", "acbd") == 2./4.);
///
/// [1]: https://siderite.dev/blog/super-fast-and-accurate-string-distance.html
pub fn sift4_simple(s1: &str, s2: &str) -> f64 {
    Sift4Simple::default().for_str(s1, s2).nval()
}

/// Calculate normalized [Sift4 distance][1] for two strings using the "common" algorithm.
///
/// A wrapper for [`Sift4Common`].
///
///     use textdistance::nstr::sift4_common;
///     assert!(sift4_common("abc", "acbd") == 2./4.);
///
/// [1]: https://siderite.dev/blog/super-fast-and-accurate-string-distance.html
pub fn sift4_common(s1: &str, s2: &str) -> f64 {
    Sift4Common::default().for_str(s1, s2).nval()
}

/// Calculate normalized [Jaro normalized similarity][1] for two strings.
///
/// A wrapper for [`Jaro`].
///
///     use textdistance::nstr::jaro;
///     assert_eq!(jaro("abc", "acbd"), 0.8055555555555555);
///
/// [1]: https://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance#Jaro_similarity
pub fn jaro(s1: &str, s2: &str) -> f64 {
    Jaro::default().for_str(s1, s2).nval()
}

/// Calculate normalized [Jaro-Winkler normalized similarity][1] for two strings.
///
/// A wrapper for [`JaroWinkler`].
///
///     use textdistance::nstr::jaro_winkler;
///     assert_eq!(jaro_winkler("abc", "acbd"), 0.825);
///
/// [1]: https://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance
pub fn jaro_winkler(s1: &str, s2: &str) -> f64 {
    JaroWinkler::default().for_str(s1, s2).nval()
}

/// Calculate normalized [Yujian-Bo normalization][1] of [Levenshtein] for two strings.
///
/// A wrapper for [`YujianBo`].
///
///     use textdistance::nstr::yujian_bo;
///     assert_eq!(yujian_bo("abc", "acbd"), 0.4444444444444444);
///
/// [1]: https://ieeexplore.ieee.org/document/4160958
pub fn yujian_bo(s1: &str, s2: &str) -> f64 {
    YujianBo::default().for_str(s1, s2).nval()
}

/// Calculate normalized [MLIPNS normalization][1] of [Hamming] for two strings.
///
/// A wrapper for [`MLIPNS`].
///
///     use textdistance::nstr::mlipns;
///     assert!(mlipns("abc", "acbd") == 0.);
///
/// [1]: https://www.sial.iias.spb.su/files/386-386-1-PB.pdf
pub fn mlipns(s1: &str, s2: &str) -> f64 {
    MLIPNS::default().for_str(s1, s2).nval()
}

/// Calculate normalized [Bag distance][1] for two strings.
///
/// A wrapper for [`Bag`].
///
///     use textdistance::nstr::bag;
///     assert!(bag("abc", "acbd") == 1./4.);
///
/// [1]: http://www-db.disi.unibo.it/research/papers/SPIRE02.pdf
#[cfg(feature = "std")]
pub fn bag(s1: &str, s2: &str) -> f64 {
    Bag::default().for_str(s1, s2).nval()
}

/// Calculate normalized [LIG3 normalization][1] of [Hamming] by [Levenshtein] for two strings.
///
/// A wrapper for [`LIG3`].
///
///     use textdistance::nstr::lig3;
///     assert_eq!(lig3("abc", "acbd"), 0.5);
///
/// [1]: https://github.com/chrislit/abydos/blob/master/abydos/distance/_lig3.py
pub fn lig3(s1: &str, s2: &str) -> f64 {
    LIG3::default().for_str(s1, s2).nval()
}

/// Calculate normalized [Jaccard normalized similarity][1] for two strings.
///
/// A wrapper for [`Jaccard`].
///
///     use textdistance::nstr::jaccard;
///     assert_eq!(jaccard("abc", "acbd"), 0.75);
///
/// [1]: https://en.wikipedia.org/wiki/Jaccard_index
#[cfg(feature = "std")]
pub fn jaccard(s1: &str, s2: &str) -> f64 {
    Jaccard::default().for_str(s1, s2).nval()
}

/// Calculate normalized [Sørensen–Dice normalized similarity][1] for two strings.
///
/// A wrapper for [`SorensenDice`].
///
///     use textdistance::nstr::sorensen_dice;
///     assert_eq!(sorensen_dice("abc", "acbd"), 0.8571428571428571);
///
/// [1]:https://en.wikipedia.org/wiki/S%C3%B8rensen%E2%80%93Dice_coefficient
#[cfg(feature = "std")]
pub fn sorensen_dice(s1: &str, s2: &str) -> f64 {
    SorensenDice::default().for_str(s1, s2).nval()
}

/// Calculate normalized [Tversky normalized similarity][1] for two strings.
///
/// A wrapper for [`Tversky`].
///
///     use textdistance::nstr::tversky;
///     assert_eq!(tversky("abc", "acbd"), 0.75);
///
/// [1]: https://en.wikipedia.org/wiki/Tversky_index
#[cfg(feature = "std")]
pub fn tversky(s1: &str, s2: &str) -> f64 {
    Tversky::default().for_str(s1, s2).nval()
}

/// Calculate normalized [Overlap normalized similarity][1] for two strings.
///
/// A wrapper for [`Overlap`].
///
///     use textdistance::nstr::overlap;
///     assert_eq!(overlap("abc", "acbd"), 1.0);
///
/// [1]: https://en.wikipedia.org/wiki/Overlap_coefficient
#[cfg(feature = "std")]
pub fn overlap(s1: &str, s2: &str) -> f64 {
    Overlap::default().for_str(s1, s2).nval()
}

/// Calculate normalized [Cosine normalized similarity][1] for two strings.
///
/// A wrapper for [`Cosine`].
///
///     use textdistance::nstr::cosine;
///     assert_eq!(cosine("abc", "acbd"), 0.8660254037844387);
///
/// [1]: https://en.wikipedia.org/wiki/Cosine_similarity
#[cfg(feature = "std")]
pub fn cosine(s1: &str, s2: &str) -> f64 {
    Cosine::default().for_str(s1, s2).nval()
}

/// Calculate normalized common prefix length for two strings.
///
/// A wrapper for [`Prefix`].
///
///     use textdistance::nstr::prefix;
///     assert!(prefix("abc", "acbd") == 1./4.); // "a"
///
pub fn prefix(s1: &str, s2: &str) -> f64 {
    Prefix::default().for_str(s1, s2).nval()
}

/// Calculate normalized common suffix length for two strings.
///
/// A wrapper for [`Suffix`].
///
///     use textdistance::nstr::suffix;
///     assert!(suffix("abcd", "axcd") == 2./4.); // "cd"
///
pub fn suffix(s1: &str, s2: &str) -> f64 {
    Suffix::default().for_str(s1, s2).nval()
}

/// Calculate normalized length distance for two strings.
///
/// A wrapper for [`Length`].
///
///     use textdistance::nstr::length;
///     assert!(length("abcd", "axc") == (4. - 3.) / 4.);
///
pub fn length(s1: &str, s2: &str) -> f64 {
    Length::default().for_str(s1, s2).nval()
}

/// Calculate normalized [Smith-Waterman similarity] for two strings.
///
/// A wrapper for [`SmithWaterman`].
///
///     use textdistance::nstr::smith_waterman;
///     assert!(smith_waterman("abc", "acbd") == 1./4.);
///
/// [Smith-Waterman similarity]: https://en.wikipedia.org/wiki/Smith%E2%80%93Waterman_algorithm
pub fn smith_waterman(s1: &str, s2: &str) -> f64 {
    SmithWaterman::default().for_str(s1, s2).nval()
}

/// Calculate normalized [Entropy]-based [normalized compression distance][1] for two strings.
///
/// A wrapper for [`EntropyNCD`].
///
///     use textdistance::nstr::entropy_ncd;
///     assert_eq!(entropy_ncd("abc", "acbd"), 0.12174985473119697);
///
/// [1]: https://en.wikipedia.org/wiki/Normalized_compression_distance
/// [Entropy]: https://en.wikipedia.org/wiki/Entropy_(information_theory)
#[cfg(feature = "std")]
pub fn entropy_ncd(s1: &str, s2: &str) -> f64 {
    EntropyNCD::default().for_str(s1, s2).nval()
}

/// Calculate normalized [Roberts similarity] for two strings.
///
/// A wrapper for [`Roberts`].
///
///     use textdistance::nstr::roberts;
///     assert_eq!(roberts("abc", "acbd"), 0.8571428571428571);
///
/// [Roberts similarity]: https://github.com/chrislit/abydos/blob/master/abydos/distance/_roberts.py
#[cfg(feature = "std")]
pub fn roberts(s1: &str, s2: &str) -> f64 {
    Roberts::default().for_str(s1, s2).nval()
}
