//! Helper functions providing the default implementation of distance/similarity algorithms for strings.

use super::Algorithm;
use super::Cosine;
use super::DamerauLevenshtein;
use super::EntropyNCD;
use super::Hamming;
use super::Jaccard;
use super::Jaro;
use super::JaroWinkler;
use super::LCSSeq;
use super::LCSStr;
use super::Levenshtein;
use super::Overlap;
use super::Prefix;
use super::RatcliffObershelp;
use super::Sift4;
use super::SorensenDice;
use super::Suffix;
use super::Tversky;
use super::YujianBo;
use super::MLIPNS;

/// Calculate unrestricted [Damerau-Levenshtein distance] for two strings.
///
/// See [DamerauLevenshtein] documentation to learn more.
///
/// [Damerau-Levenshtein distance]: https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance
pub fn damerau_levenshtein(s1: &str, s2: &str) -> usize {
    DamerauLevenshtein::default().for_str(s1, s2).val()
}

/// Calculate restricted [Damerau-Levenshtein distance] for two strings.
///
/// See [DamerauLevenshtein] documentation to learn more.
///
/// [Damerau-Levenshtein distance]: https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance
pub fn damerau_levenshtein_restricted(s1: &str, s2: &str) -> usize {
    let a = DamerauLevenshtein {
        restricted: true,
        ..Default::default()
    };
    a.for_str(s1, s2).val()
}

/// Calculate [Hamming distance] for two strings.
///
/// See [Hamming] documentation to learn more.
///
/// [Hamming distance]: https://en.wikipedia.org/wiki/Hamming_distance
pub fn hamming(s1: &str, s2: &str) -> usize {
    Hamming::default().for_str(s1, s2).val()
}

/// Calculate the length of the [Longest Common SubSequence] for two strings.
///
/// See [LCSSeq] documentation to learn more.
///
/// [Longest Common SubSequence]: https://en.wikipedia.org/wiki/Longest_common_subsequence
pub fn lcsseq(s1: &str, s2: &str) -> usize {
    LCSSeq::default().for_str(s1, s2).val()
}

/// Calculate the length of the [Longest Common SubString] for two strings.
///
/// See [LCSStr] documentation to learn more.
///
/// [Longest Common SubString]: https://en.wikipedia.org/wiki/Longest_common_substring
pub fn lcsstr(s1: &str, s2: &str) -> usize {
    LCSStr::default().for_str(s1, s2).val()
}

/// Calculate [Levenshtein distance] for two strings.
///
/// See [Levenshtein] documentation to learn more.
///
/// [Levenshtein distance]: https://en.wikipedia.org/wiki/Levenshtein_distance
pub fn levenshtein(s1: &str, s2: &str) -> usize {
    Levenshtein::default().for_str(s1, s2).val()
}

/// Calculate [Ratcliff-Obershelp normalized similarity] for two strings.
///
/// See [RatcliffObershelp] documentation to learn more.
///
/// [Ratcliff-Obershelp normalized similarity]: https://en.wikipedia.org/wiki/Gestalt_pattern_matching
pub fn ratcliff_obershelp(s1: &str, s2: &str) -> f64 {
    RatcliffObershelp::default().for_str(s1, s2).nval()
}

pub fn sift4(s1: &str, s2: &str) -> usize {
    Sift4::default().for_str(s1, s2).val()
}

pub fn jaro(s1: &str, s2: &str) -> f64 {
    Jaro::default().for_str(s1, s2).nval()
}

pub fn jaro_winkler(s1: &str, s2: &str) -> f64 {
    JaroWinkler::default().for_str(s1, s2).nval()
}

pub fn yujian_bo(s1: &str, s2: &str) -> f64 {
    YujianBo::default().for_str(s1, s2).nval()
}

pub fn mlipns(s1: &str, s2: &str) -> usize {
    MLIPNS::default().for_str(s1, s2).val()
}

pub fn jaccard(s1: &str, s2: &str) -> f64 {
    Jaccard::default().for_str(s1, s2).nval()
}

pub fn sorensen_dice(s1: &str, s2: &str) -> f64 {
    SorensenDice::default().for_str(s1, s2).nval()
}

pub fn tversky(s1: &str, s2: &str) -> f64 {
    Tversky::default().for_str(s1, s2).nval()
}

pub fn overlap(s1: &str, s2: &str) -> f64 {
    Overlap::default().for_str(s1, s2).nval()
}

pub fn cosine(s1: &str, s2: &str) -> f64 {
    Cosine::default().for_str(s1, s2).nval()
}

pub fn prefix(s1: &str, s2: &str) -> usize {
    Prefix::default().for_str(s1, s2).val()
}

pub fn suffix(s1: &str, s2: &str) -> usize {
    Suffix::default().for_str(s1, s2).val()
}

pub fn entropy_ncd(s1: &str, s2: &str) -> f64 {
    EntropyNCD::default().for_str(s1, s2).nval()
}