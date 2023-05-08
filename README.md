# textdistance.rs

[ [github.com](https://github.com/life4/textdistance.rs) ]
[ [docs.rs](https://docs.rs/textdistance/) ]
[ [crates.io](crates.io/crates/textdistance) ]

Rust library with different algorithms to compare how similar two sequences are.

Features:

+ 💪 Based on popular and battle-tested [textdistance](https://github.com/life4/textdistance) Python library (and written by the same author).
+ 📚 Contains 20+ algorithms for all purposes.
+ 🔬 Includes state of the art algorithms like `EntropyNCD` and `Sift4`.
+ 🪶 Zero dependency.
+ 🔨 Works with any iterators, including bytes, code points, unicode grapheme clusters, words, and numbers.
+ ❤️ Friendly and consistent API for all algorithms.
+ 📏 Optional normalization of the result on 0.0-1.0 interval.
+ 🛡 No unsafe code.
+ 🦀 Pure Rust.

## Available algorithms

Edit-based:

1. `DamerauLevenshtein`, both optimal string alignment and restricted.
1. `Hamming`
1. `Jaro`
1. `JaroWinkler`
1. `Levenshtein`
1. `Sift4Common`
1. `Sift4Simple`
1. `SmithWaterman`

Token-based:

1. `Bag`
1. `Cosine` (aka Orchini, Tucker, Otsuka–Ochiai)
1. `EntropyNCD` (Entropy-based Normalized Compression Distance)
1. `Jaccard` (aka Tanimoto, Critical Success Index)
1. `Overlap` (aka Szymkiewicz–Simpson)
1. `Roberts`
1. `SorensenDice` (aka F1, Czekanowski, Zijdenbos)
1. `Tversky`

Sequence-based:

1. `LCSSeq` (Longest Common SubSequence)
1. `LCSStr` (Longest Common SubString)
1. `RatcliffObershelp` (aka Gestalt pattern matching)

Naive:

1. `Prefix`
1. `Suffix`
1. `Length`

Normalization for other metrics:

1. `LIG3` normallization for `Hamming` by `Levenshtein`
1. `MLIPNS` normallization for `Hamming`
1. `YujianBo` normallization for `Levenshtein`

## Installation

```shell
cargo add textdistance
```

## Usage

...

## Unicode support

...

## Versioning

We stick to SemVer:

1. The **patch** number is for bug fixes. It is possible that the results of an algorithm will change in some corner cases if we found that the previous implementation doesn't match the algorithm described in the original paper.
1. The **minor** number is for new algorithms and features.
1. The **major** number is for big changes in the API. We try to avoid breaking stuff but we prefer to provide a friendly and convenient API over keeping a backward compatibility.

## Limitations

+ In the original textdisance, most of the algorithms are adjusted to work on any number of the input sequences. However, Rust doesn't support variadic arguments, so all algorithms currently are implemented only for exactly two inputs.
+ All algorithms in the crate implement the same `Algorithm` trait. Hence metrics that have additional limitations on the input sequence elements beyond `Eq` (like Editex and MRA that work only with ASCII letters) currently cannot be implemented.
+ Most of the implemented algorithms have certain properties (like [commutative property](https://en.wikipedia.org/wiki/Commutative_property)) that make their behavior more like what you would expect and make normalization simple. So, I haven't implemented yet Needleman-Wunsch and Gotoh, mostly because they are tricky to normalize and I'm still not 100% sure that I did it correctly in the original textdistance.

## Acknowledgments

There are the libraries that I used as a reference implementation and the source of test cases:

+ Python: [textdistance](https://github.com/life4/textdistance), [abydos](https://github.com/chrislit/abydos), [jellyfish](https://github.com/jamesturk/jellyfish).
+ JS: [talisman](https://github.com/Yomguithereal/talisman).
+ Rust: [strsim](https://github.com/dguo/strsim-rs), [distance](https://github.com/mbrlabs/distance), [levenshtein-rs](https://github.com/wooorm/levenshtein-rs).

## Testing locally

To run everything locally, all you need is rust, python, and [task](https://taskfile.dev/installation/). Execute `task all` to run all code formatters, linters, and tests.
