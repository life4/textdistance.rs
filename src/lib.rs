pub mod textdistance {
    mod hamming;
    mod smith_waterman;

    pub use self::hamming::hamming;
    pub use self::smith_waterman::smith_waterman;
}
