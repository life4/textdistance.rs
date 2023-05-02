//! Helper functions providing the default implementation of distance/similarity algorithms for strings.

use super::Algorithm;
use super::Bag;
use super::Cosine;
use super::DamerauLevenshtein;
use super::EntropyNCD;
use super::Hamming;
use super::Jaccard;
use super::Jaro;
use super::JaroWinkler;
use super::LCSSeq;
use super::LCSStr;
use super::Length;
use super::Levenshtein;
use super::Overlap;
use super::Prefix;
use super::RatcliffObershelp;
use super::Sift4;
use super::SorensenDice;
use super::Suffix;
use super::Tversky;
use super::YujianBo;
use super::LIG3;
use super::MLIPNS;

/// Calculate unrestricted [Damerau-Levenshtein distance][1] for two strings.
///
/// A wrapper for [DamerauLevenshtein].
///
/// [1]: https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance
pub fn damerau_levenshtein(s1: &str, s2: &str) -> usize {
    DamerauLevenshtein::default().for_str(s1, s2).val()
}

/// Calculate restricted [Damerau-Levenshtein distance][1] for two strings.
///
/// A wrapper for [DamerauLevenshtein].
///
/// [1]: https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance
pub fn damerau_levenshtein_restricted(s1: &str, s2: &str) -> usize {
    let a = DamerauLevenshtein {
        restricted: true,
        ..Default::default()
    };
    a.for_str(s1, s2).val()
}

/// Calculate [Hamming distance][1] for two strings.
///
/// A wrapper for [Hamming].
///
/// [1]: https://en.wikipedia.org/wiki/Hamming_distance
pub fn hamming(s1: &str, s2: &str) -> usize {
    Hamming::default().for_str(s1, s2).val()
}

/// Calculate the length of the [Longest Common SubSequence][1] for two strings.
///
/// A wrapper for [LCSSeq].
///
/// [1]: https://en.wikipedia.org/wiki/Longest_common_subsequence
pub fn lcsseq(s1: &str, s2: &str) -> usize {
    LCSSeq::default().for_str(s1, s2).val()
}

/// Calculate the length of the [Longest Common SubString][1] for two strings.
///
/// A wrapper for [LCSStr].
///
/// [1]: https://en.wikipedia.org/wiki/Longest_common_substring
pub fn lcsstr(s1: &str, s2: &str) -> usize {
    LCSStr::default().for_str(s1, s2).val()
}

/// Calculate [Levenshtein distance][1] for two strings.
///
/// A wrapper for [Levenshtein].
///
/// [1]: https://en.wikipedia.org/wiki/Levenshtein_distance
pub fn levenshtein(s1: &str, s2: &str) -> usize {
    Levenshtein::default().for_str(s1, s2).val()
}

/// Calculate [Ratcliff-Obershelp normalized similarity][1] for two strings.
///
/// A wrapper for [RatcliffObershelp].
///
/// [1]: https://en.wikipedia.org/wiki/Gestalt_pattern_matching
pub fn ratcliff_obershelp(s1: &str, s2: &str) -> f64 {
    RatcliffObershelp::default().for_str(s1, s2).nval()
}

/// Calculate [Sift4 distance][1] for two strings.
///
/// A wrapper for [Sift4].
///
/// [1]: https://siderite.dev/blog/super-fast-and-accurate-string-distance.html
pub fn sift4(s1: &str, s2: &str) -> usize {
    Sift4::default().for_str(s1, s2).val()
}

/// Calculate [Jaro normalized similarity][1] for two strings.
///
/// A wrapper for [Jaro].
///
/// [1]: https://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance#Jaro_similarity
pub fn jaro(s1: &str, s2: &str) -> f64 {
    Jaro::default().for_str(s1, s2).nval()
}

/// Calculate [Jaro-Winkler normalized similarity][1] for two strings.
///
/// A wrapper for [JaroWinkler].
///
/// [1]: https://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance
pub fn jaro_winkler(s1: &str, s2: &str) -> f64 {
    JaroWinkler::default().for_str(s1, s2).nval()
}

/// Calculate [Yujian-Bo normalization][1] of [Levenshtein] for two strings.
///
/// A wrapper for [YujianBo].
///
/// [1]: https://ieeexplore.ieee.org/document/4160958
pub fn yujian_bo(s1: &str, s2: &str) -> f64 {
    YujianBo::default().for_str(s1, s2).nval()
}

/// Calculate [MLIPNS normalization][1] of [Hamming] for two strings.
///
/// A wrapper for [MLIPNS].
///
/// [1]: https://www.sial.iias.spb.su/files/386-386-1-PB.pdf
pub fn mlipns(s1: &str, s2: &str) -> usize {
    MLIPNS::default().for_str(s1, s2).val()
}

/// Calculate [Bag distance][1] for two strings.
///
/// A wrapper for [Bag].
///
/// [1]: http://www-db.disi.unibo.it/research/papers/SPIRE02.pdf
pub fn bag(s1: &str, s2: &str) -> usize {
    Bag::default().for_str(s1, s2).val()
}

/// Calculate [LIG3 normalization][1] of [Hamming] by [Levenshtein] for two strings.
///
/// A wrapper for [LIG3].
///
/// [1]: https://github.com/chrislit/abydos/blob/master/abydos/distance/_lig3.py
pub fn lig3(s1: &str, s2: &str) -> f64 {
    LIG3::default().for_str(s1, s2).nval()
}

/// Calculate [Jaccard normalized similarity][1] for two strings.
///
/// A wrapper for [Jaccard].
///
/// [1]: https://en.wikipedia.org/wiki/Jaccard_index
pub fn jaccard(s1: &str, s2: &str) -> f64 {
    Jaccard::default().for_str(s1, s2).nval()
}

/// Calculate [Sørensen–Dice normalized similarity][1] for two strings.
///
/// A wrapper for [SorensenDice].
///
/// [1]:https://en.wikipedia.org/wiki/S%C3%B8rensen%E2%80%93Dice_coefficient
pub fn sorensen_dice(s1: &str, s2: &str) -> f64 {
    SorensenDice::default().for_str(s1, s2).nval()
}

/// Calculate [Tversky normalized similarity][1] for two strings.
///
/// A wrapper for [Tversky].
///
/// [1]: https://en.wikipedia.org/wiki/Tversky_index
pub fn tversky(s1: &str, s2: &str) -> f64 {
    Tversky::default().for_str(s1, s2).nval()
}

/// Calculate [Overlap normalized similarity][1] for two strings.
///
/// A wrapper for [Overlap].
///
/// [1]: https://en.wikipedia.org/wiki/Overlap_coefficient
pub fn overlap(s1: &str, s2: &str) -> f64 {
    Overlap::default().for_str(s1, s2).nval()
}

/// Calculate [Cosine normalized similarity][1] for two strings.
///
/// A wrapper for [Cosine].
///
/// [1]: https://en.wikipedia.org/wiki/Cosine_similarity
pub fn cosine(s1: &str, s2: &str) -> f64 {
    Cosine::default().for_str(s1, s2).nval()
}

/// Calculate prefix similarity for two strings.
///
/// A wrapper for [Prefix].
pub fn prefix(s1: &str, s2: &str) -> usize {
    Prefix::default().for_str(s1, s2).val()
}

/// Calculate suffix similarity for two strings.
///
/// A wrapper for [Suffix].
pub fn suffix(s1: &str, s2: &str) -> usize {
    Suffix::default().for_str(s1, s2).val()
}

/// Calculate length similarity for two strings.
///
/// A wrapper for [Length].
pub fn length(s1: &str, s2: &str) -> usize {
    Length::default().for_str(s1, s2).val()
}

/// Calculate Entropy-based [normalized compression distance][1] for two strings.
///
/// A wrapper for [EntropyNCD].
///
/// [1]: https://en.wikipedia.org/wiki/Normalized_compression_distance
pub fn entropy_ncd(s1: &str, s2: &str) -> f64 {
    EntropyNCD::default().for_str(s1, s2).nval()
}
