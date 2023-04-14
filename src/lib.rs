pub mod textdistance {
    mod algorithm;
    mod hamming;
    mod lcsseq;
    mod lcsstr;

    pub use self::algorithm::Algorithm;
    pub use self::hamming::{hamming, Hamming};
    pub use self::lcsseq::{lcsseq, LCSSeq};
    pub use self::lcsstr::{lcsstr, LCSStr};
}
