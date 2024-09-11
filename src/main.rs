#![allow(clippy::cast_precision_loss)]

use core::borrow::Borrow;
use textdistance::str;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let alg_name = args.get(1).expect("algorithm name is required");
    let s1 = args.get(2).expect("first text is required");
    let s2 = args.get(3).expect("second text is required");

    let res: f64 = match alg_name.to_lowercase().borrow() {
        "hamming" => str::hamming(s1, s2) as f64,
        "roberts" => str::roberts(s1, s2),
        _ => panic!("unknown algorithm name"),
    };
    println!("{res}");
}
