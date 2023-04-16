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
            // assert_eq!(alg.normalized_distance("", ""), 0.);
            // assert_eq!(alg.normalized_similarity("", ""), 1.);
        }
    }
}
