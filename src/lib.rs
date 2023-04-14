pub mod textdistance {
    mod algorithm;
    mod hamming;
    mod lcsseq;
    mod lcsstr;
    mod smith_waterman;

    pub use self::algorithm::Algorithm;
    pub use self::hamming::{hamming, Hamming};
    pub use self::lcsseq::{lcsseq, LCSSeq};
    pub use self::lcsstr::{lcsstr, LCSStr};
    pub use self::smith_waterman::smith_waterman;
}
