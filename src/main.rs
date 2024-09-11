use core::borrow::Borrow;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let alg_name = args.get(1).expect("algorithm name is required");
    let s1 = args.get(2).expect("first text is required");
    let s2 = args.get(3).expect("second text is required");

    #[allow(clippy::cast_precision_loss)]
    let res: f64 = match alg_name.to_lowercase().borrow() {
        #[cfg(feature = "std")]
        "damerau_levenshtein" => textdistance::str::damerau_levenshtein(s1, s2) as f64,
        #[cfg(feature = "std")]
        "damerau_levenshtein_restricted" => {
            textdistance::str::damerau_levenshtein_restricted(s1, s2) as f64
        }
        "hamming" => textdistance::str::hamming(s1, s2) as f64,
        "lcsseq" => textdistance::str::lcsseq(s1, s2) as f64,
        "lcsstr" => textdistance::str::lcsstr(s1, s2) as f64,
        "levenshtein" => textdistance::str::levenshtein(s1, s2) as f64,
        "ratcliff_obershelp" => textdistance::str::ratcliff_obershelp(s1, s2),
        "sift4_simple" => textdistance::str::sift4_simple(s1, s2) as f64,
        "sift4_common" => textdistance::str::sift4_common(s1, s2) as f64,
        "jaro" => textdistance::str::jaro(s1, s2),
        "jaro_winkler" => textdistance::str::jaro_winkler(s1, s2),
        "yujian_bo" => textdistance::str::yujian_bo(s1, s2),
        "mlipns" => textdistance::str::mlipns(s1, s2) as f64,
        #[cfg(feature = "std")]
        "bag" => textdistance::str::bag(s1, s2) as f64,
        "lig3" => textdistance::str::lig3(s1, s2),
        #[cfg(feature = "std")]
        "jaccard" => textdistance::str::jaccard(s1, s2),
        #[cfg(feature = "std")]
        "sorensen_dice" => textdistance::str::sorensen_dice(s1, s2),
        #[cfg(feature = "std")]
        "tversky" => textdistance::str::tversky(s1, s2),
        #[cfg(feature = "std")]
        "overlap" => textdistance::str::overlap(s1, s2),
        #[cfg(feature = "std")]
        "cosine" => textdistance::str::cosine(s1, s2),
        "prefix" => textdistance::str::prefix(s1, s2) as f64,
        "suffix" => textdistance::str::suffix(s1, s2) as f64,
        "length" => textdistance::str::length(s1, s2) as f64,
        "smith_waterman" => textdistance::str::smith_waterman(s1, s2) as f64,
        #[cfg(feature = "std")]
        "entropy_ncd" => textdistance::str::entropy_ncd(s1, s2),
        #[cfg(feature = "std")]
        "roberts" => textdistance::str::roberts(s1, s2),
        _ => panic!("unknown algorithm name"),
    };
    println!("{res}");
}
