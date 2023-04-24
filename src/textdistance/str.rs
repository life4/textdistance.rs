use super::algorithm::Algorithm;
use super::damerau_levenshtein::DamerauLevenshtein;
use super::hamming::Hamming;
use super::lcsseq::LCSSeq;
use super::lcsstr::LCSStr;
use super::levenshtein::Levenshtein;
use super::ratcliff_obershelp::RatcliffObershelp;

pub fn damerau_levenshtein(s1: &str, s2: &str) -> usize {
    let a: DamerauLevenshtein = Default::default();
    a.for_str(s1, s2).val()
}

pub fn damerau_levenshtein_restricted(s1: &str, s2: &str) -> usize {
    let a = DamerauLevenshtein {
        restricted: true,
        ..Default::default()
    };
    a.for_str(s1, s2).val()
}

pub fn hamming(s1: &str, s2: &str) -> usize {
    let a: Hamming = Default::default();
    a.for_str(s1, s2).val()
}

pub fn lcsseq(s1: &str, s2: &str) -> usize {
    let a: LCSSeq = Default::default();
    a.for_str(s1, s2).val()
}

pub fn lcsstr(s1: &str, s2: &str) -> usize {
    let a: LCSStr = Default::default();
    a.for_str(s1, s2).val()
}

pub fn levenshtein(s1: &str, s2: &str) -> usize {
    let a: Levenshtein = Default::default();
    a.for_str(s1, s2).val()
}

pub fn ratcliff_obershelp(s1: &str, s2: &str) -> f64 {
    let a: RatcliffObershelp = Default::default();
    a.for_str(s1, s2).nval()
}
