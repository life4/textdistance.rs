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
