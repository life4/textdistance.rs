pub mod textdistance {
    mod algorithm;
    mod damerau_levenshtein;
    mod hamming;
    mod lcsseq;
    mod lcsstr;
    mod levenshtein;
    mod ratcliff_obershelp;

    pub use self::algorithm::{Algorithm, Result};
    pub use self::damerau_levenshtein::{damerau_levenshtein, DamerauLevenshtein};
    pub use self::hamming::{hamming, Hamming};
    pub use self::lcsseq::{lcsseq, LCSSeq};
    pub use self::lcsstr::{lcsstr, LCSStr};
    pub use self::levenshtein::{levenshtein, Levenshtein};
    pub use self::ratcliff_obershelp::{ratcliff_obershelp, RatcliffObershelp};
}

#[cfg(test)]
mod tests {
    use crate::textdistance::*;
    use proptest::prelude::*;

    fn hamming(s1: &str, s2: &str) -> Result {
        let a: Hamming = Default::default();
        a.for_str(s1, s2)
    }

    fn lcsseq(s1: &str, s2: &str) -> Result {
        let a: LCSSeq = Default::default();
        a.for_str(s1, s2)
    }

    fn lcsstr(s1: &str, s2: &str) -> Result {
        let a: LCSStr = Default::default();
        a.for_str(s1, s2)
    }

    fn ratcliff_obershelp(s1: &str, s2: &str) -> Result {
        let a: RatcliffObershelp = Default::default();
        a.for_str(s1, s2)
    }

    fn levenshtein(s1: &str, s2: &str) -> Result {
        let a: Levenshtein = Default::default();
        a.for_str(s1, s2)
    }

    fn damerau_levenshtein(s1: &str, s2: &str) -> Result {
        let a: DamerauLevenshtein = Default::default();
        a.for_str(s1, s2)
    }

    type AlgFn = dyn Fn(&str, &str) -> Result;

    fn get_algs() -> Vec<Box<AlgFn>> {
        vec![
            Box::new(damerau_levenshtein),
            Box::new(hamming),
            Box::new(lcsseq),
            Box::new(lcsstr),
            Box::new(levenshtein),
            Box::new(ratcliff_obershelp),
        ]
    }

    #[test]
    fn basic() {
        for alg in get_algs() {
            assert_eq!(alg("", "").dist(), 0);
            assert!(alg("ab", "cde").dist() > 0);
            assert!(alg("spam", "qwer").sim() == 0);
            assert_eq!(alg("", "").ndist(), 0.);
            assert_eq!(alg("", "").nsim(), 1.);
        }
    }

    proptest! {
        #[test]
        fn prop(s1 in ".*", s2 in ".*") {
            for alg in get_algs() {
                let d = alg(&s1, &s2).dist();
                let s = alg(&s1, &s2).sim();

                let nd = alg(&s1, &s2).ndist();
                assert!(nd >= 0.);
                assert!(nd <= 1.);

                let ns = alg(&s1, &s2).nsim();
                assert!(ns >= 0.);
                assert!(ns <= 1.);

                assert!((ns + nd) > 0.9999999, "{} + {} == 1", nd, ns);
                assert!((ns + nd) < 1.0000001, "{} + {} == 1", nd, ns);

                if d < s {
                    assert!(nd < ns, "{} < {}", nd, ns);
                } else if d > s {
                    assert!(nd > ns, "{} > {}", nd, ns);
                } else if !s1.is_empty() && !s2.is_empty() {
                    assert!(nd == ns, "{} == {}", nd, ns);
                }
            }
        }

        fn prop_same(s in ".*") {
            for alg in get_algs() {
                let nd = alg(&s, &s).ndist();
                assert_eq!(nd, 0., "{} == 0.0", nd);

                let ns = alg(&s, &s).nsim();
                assert_eq!(ns, 1., "{} == 1.0", ns);
            }
        }
    }
}
