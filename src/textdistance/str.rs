//! Helper functions providing the default implementation of distance/similarity algorithms for strings.

use super::algorithm::Algorithm;
use super::damerau_levenshtein::DamerauLevenshtein;
use super::hamming::Hamming;
use super::lcsseq::LCSSeq;
use super::lcsstr::LCSStr;
use super::levenshtein::Levenshtein;
use super::ratcliff_obershelp::RatcliffObershelp;

/// Calculate unrestricted [Damerau-Levenshtein distance] for two strings.
///
/// See [DamerauLevenshtein] documentation to learn more.
///
/// [Damerau-Levenshtein distance]: https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance
pub fn damerau_levenshtein(s1: &str, s2: &str) -> usize {
    let a: DamerauLevenshtein = Default::default();
    a.for_str(s1, s2).val()
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
    let a: Hamming = Default::default();
    a.for_str(s1, s2).val()
}

/// Calculate the length of the [Longest Common SubSequence] for two strings.
///
/// See [LCSSeq] documentation to learn more.
///
/// [Longest Common SubSequence]: https://en.wikipedia.org/wiki/Longest_common_subsequence
pub fn lcsseq(s1: &str, s2: &str) -> usize {
    let a: LCSSeq = Default::default();
    a.for_str(s1, s2).val()
}

/// Calculate the length of the [Longest Common SubString] for two strings.
///
/// See [LCSStr] documentation to learn more.
///
/// [Longest Common SubString]: https://en.wikipedia.org/wiki/Longest_common_substring
pub fn lcsstr(s1: &str, s2: &str) -> usize {
    let a: LCSStr = Default::default();
    a.for_str(s1, s2).val()
}

/// Calculate [Levenshtein distance] for two strings.
///
/// See [Levenshtein] documentation to learn more.
///
/// [Levenshtein distance]: https://en.wikipedia.org/wiki/Levenshtein_distance
pub fn levenshtein(s1: &str, s2: &str) -> usize {
    let a: Levenshtein = Default::default();
    a.for_str(s1, s2).val()
}

/// Calculate [Ratcliff-Obershelp normalized similarity] for two strings.
///
/// See [RatcliffObershelp] documentation to learn more.
///
/// [Ratcliff-Obershelp normalized similarity]: https://en.wikipedia.org/wiki/Gestalt_pattern_matching
pub fn ratcliff_obershelp(s1: &str, s2: &str) -> f64 {
    let a: RatcliffObershelp = Default::default();
    a.for_str(s1, s2).nval()
}
