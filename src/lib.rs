pub mod textdistance {
    mod hamming;
    mod lcsseq;
    mod smith_waterman;

    pub use self::hamming::hamming;
    pub use self::lcsseq::lcsseq;
    pub use self::smith_waterman::smith_waterman;
}
