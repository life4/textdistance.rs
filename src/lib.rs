pub mod textdistance {
    mod algorithm;
    mod hamming;
    mod lcsseq;
    mod smith_waterman;

    pub use self::algorithm::Algorithm;
    pub use self::hamming::{hamming, Hamming};
    pub use self::lcsseq::lcsseq;
    pub use self::smith_waterman::smith_waterman;
}
