# textdistance.rs

Rust library with different algorithms to compare how similar two sequences are.

Features:

+ Based on popular and battle-tested [textdistance](https://github.com/life4/textdistance) Python library (and written by the same author).
+ Includes state of the art algorithms like `EntropyNCD` and `Sift4`.
+ Zero dependency.
+ Works with any iterators, including bytes, code points, unicode grapheme clusters, words, and numbers.
+ Friendly and consistent API for all algorithms.
+ Optional normalization of the result on 0.0-1.0 interval.
+ No unsafe code.
+ Pure Rust.

## Available algorithms

Edit-based:

1. `DamerauLevenshtein`, both optimal string alignment and restricted.
1. `Hamming`
1. `Jaro`
1. `JaroWinkler`
1. `Levenshtein`
1. `Sift4`

Token-based:

1. `Cosine` (aka Orchini, Tucker, Otsuka–Ochiai)
1. `EntropyNCD` (Entropy-based Normalized Compression Distance)
1. `Jaccard` (aka Tanimoto)
1. `Overlap` (aka Szymkiewicz–Simpson)
1. `SorensenDice` (aka F1, Czekanowski, Zijdenbos)
1. `Tversky`

Sequence-based:

1. `LCSSeq` (Longest Common SubSequence)
1. `LCSStr` (Longest Common SubString)
1. `RatcliffObershelp` (aka Gestalt pattern matching)

Naive:

1. `Prefix`
1. `Suffix`

Normalization for other metrics:

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
