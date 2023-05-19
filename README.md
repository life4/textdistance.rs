# textdistance.rs

[ [github.com](https://github.com/life4/textdistance.rs) ]
[ [docs.rs](https://docs.rs/textdistance/) ]
[ [crates.io](crates.io/crates/textdistance) ]

Rust library with lots of different algorithms to compare how similar two sequences are.

Features:

+ 💪 Based on popular and battle-tested [textdistance](https://github.com/life4/textdistance) Python library (and written by the same author).
+ 📚 Contains 20+ algorithms for all purposes.
+ 🔬 Includes state-of-the-art algorithms like `EntropyNCD` and `Sift4`.
+ 🪶 Zero-dependency.
+ 🔨 Works with any iterators, including bytes, code points, Unicode grapheme clusters, words, and numbers.
+ ❤️ Friendly and consistent API for all algorithms.
+ 📏 Optional normalization of the result on the 0.0-1.0 interval.
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

1. `LIG3` normalization for `Hamming` by `Levenshtein`
1. `MLIPNS` normalization for `Hamming`
1. `YujianBo` normalization for `Levenshtein`

## Installation

```shell
cargo add textdistance
```

## Usage

The `textdistance::str` module provides shortcut functions for each algorithm for calculating the distance/similarity between two strings:

```rust
use textdistance::str::damerau_levenshtein;
assert!(damerau_levenshtein("abc", "acbd") == 2);
```

The `textdistance::nstr` module is the same but all algorithms return a normalized value (between 0.0 and 1.0):

```rust
use textdistance::nstr::damerau_levenshtein;
assert!(damerau_levenshtein("abc", "acbd") == 2./4.);
```

For more advanced usage, each algorithm is provided as a struct implementing the `Algorithm` trait:

```rust
use textdistance::{Algorithm, DamerauLevenshtein};

let a = DamerauLevenshtein::default();
let r = a.for_str("abc", "acbd");
assert!(r.val() == 2);
assert!(r.nval() == 2./4.);
```

1. The `Algorithm` trait provides `for_str`, `for_vec`, and `for_iter` to calculate the result for two strings, vectors (slices), or iterators respectively. In addition, there are `for_words` and `for_bigrams` methods that split the text into words or bigrams respectively before calculating the distance.
1. Each method returns a `textdistance::Result` that provides methods to get absolute (`val`) or normalized (`nval`) value of the metric, distance (`dist` and `ndist`), or similarity (`sim` and `nsim`).

## Unicode support

The `for_str` method (and so all functions in the `str` and `nstr` modules) uses `String.chars` to split the string and then runs it through the `for_iter` method. So, `é` will be considered two distinct characters ("latin small letter e" and "combining acute accent"). Usually, that's ok and this is how Python works. You can read more in [the official Rust documentation](https://doc.rust-lang.org/std/primitive.char.html#representation).

If you want `é` to be considered as a single symbol, use the [unicode-segmentation](https://crates.io/crates/unicode-segmentation) crate:

```rust
use textdistance::{Algorithm, DamerauLevenshtein};
use unicode_segmentation::UnicodeSegmentation;

let s1 = "a̐éö̲\r\n";
let s2 = "éa̐ö̲\r\n";
let g1 = s1.graphemes(true);
let g2 = s2.graphemes(true);
let a = DamerauLevenshtein::default();
let r = a.for_iter(g1, g2);
assert!(r.val() == 1);
```

## Choosing the algorithm

The algorithm to use depends on your use case. First, you need to decide on the algorithm category:

1. Edit-based algorithms work well on short sequences for detecting typos and minor changes.
1. Token-based algorithms work well on longer sequences for comparing long texts with noticeable modifications.
1. Sequence-based algorithms work well for calculating diff size between the original and the changed version of the sequence.

If you go with edit-based, the next thing is to decide what kind of changes you need to detect:

+ ✏️ Substitution. One character is replaced by another.
+ ➕ Addition. A new character is added.
+ 🗑 Deletion. A character is removed.
+ 🔄 Transposition. Two sequential characters are swapped.

| alg                   | sub | add | del | trans |
| --------------------- | --- | --- | --- | ----- |
| `Hamming`             | ✅  | ❌  | ❌  | ❌    |
| `Jaro`                | ❌  | ❌  | ❌  | ✅    |
| `JaroWinkler`         | ❌  | ❌  | ❌  | ✅    |
| `Sift4`               | ❌  | ❌  | ❌  | ✅    |
| `Levenshtein`         | ✅  | ✅  | ✅  | ❌    |
| `DamerauLevenshtein`  | ✅  | ✅  | ✅  | ✅    |

+ `Hamming` is the fastest one but detects only substitutions.
+ `Sift4` is very fast but not as well-known and battle-tested as other algorithms.
+ `Jaro` is slower than `Sift4` but well-known and battle-tested.
+ `JaroWinkler` is like `Jaro` but gives more weight to strings with a matching prefix.
+ `Levenshtein` detects everything but transpositions and faster than `DamerauLevenshtein` (but slower than other algorithms).
+ `DamerauLevenshtein` ticks all the boxes but very slow.

There are some use cases:

+ `Jaro` is included in the Elixir standard library ([String.jaro_distance](https://hexdocs.pm/elixir/1.12/String.html#jaro_distance/2)). It is used by the compiler and by mix (cargo for Elixir) to provide the "did you mean?" functionality for typos in module or command names.
+ `RatcliffObershelp` variation is included in the Python standard library ([difflib.SequenceMatcher](https://docs.python.org/3/library/difflib.html#difflib.SequenceMatcher)).

## Benchmarks

Legend:

+ 🐌 is very slow (> 5 ms)
+ 🐢 is slow (> 1 ms)
+ 🐇 is fast (> 500 µs)
+ 🐎 is very fast (< 500 µs)

| algorithm          | time      |
| ------------------ | --------- |
| bag                | 🐇 523.06 µs |
| cosine             | 🐇 508.59 µs |
| damerau_levenshtein | 🐌 41.938 ms |
| damerau_levenshtein_restricted | 🐌 10.301 ms |
| entropy_ncd        | 🐇 731.68 µs |
| hamming            | 🐎 19.203 µs |
| jaccard            | 🐇 580.79 µs |
| jaro_winkler       | 🐢 1.7174 ms |
| jaro               | 🐢 1.7148 ms |
| lcsseq             | 🐌 7.4349 ms |
| lcsstr             | 🐢 3.2658 ms |
| length             | 🐎 2.5300 µs |
| levenshtein        | 🐢 4.5999 ms |
| lig3               | 🐌 6.5563 ms |
| mlipns             | 🐎 20.625 µs |
| overlap            | 🐇 513.76 µs |
| prefix             | 🐎 22.473 µs |
| ratcliff_obershelp | 🐌 36.308 ms |
| roberts            | 🐇 714.79 µs |
| sift4_common       | 🐎 238.86 µs |
| sift4_simple       | 🐎 143.69 µs |
| smith_waterman     | 🐌 9.5146 ms |
| sorensen_dice      | 🐇 510.75 µs |
| suffix             | 🐎 38.821 µs |
| tversky            | 🐇 512.41 µs |
| yujian_bo          | 🐢 4.6044 ms |

The benchmarks are powered by [criterion](https://github.com/bheisler/criterion.rs) and live in the [benches](./benches/) directory. They are quite simple: grab 10 [open-source licenses](https://github.com/github/choosealicense.com/tree/gh-pages/_licenses), take a 200 chars prefix from each, and cross-compare these prefixes. The numbers might be very different for a different kind of input, length of the input, when comparing words rather than characters, or running the benchmarks on a different machine. The goal of these benchmarks is to provide basic guidance rather than give a definitive answer. If performance is critical for your application, I encourage you to make your benchmarks on the real data you have.

## Versioning

We stick to [SemVer](https://semver.org/):

1. The **patch** number is for bug fixes. The results of an algorithm may change in some corner cases if we found that the previous implementation doesn't match the algorithm described in the original paper.
1. The **minor** number is for new algorithms and features.
1. The **major** number is for big changes in the API. We try to avoid breaking stuff but we prefer to provide a friendly and convenient API over keeping a backward compatibility.

## Limitations

+ In the original textdisance, most of the algorithms are adjusted to work on any number of the input sequences. However, Rust doesn't support variadic arguments, so all algorithms currently are implemented only for exactly two inputs.
+ All algorithms in the crate implement the same `Algorithm` trait. Hence metrics that have additional limitations on the input sequence elements beyond `Eq` (like Editex and MRA that work only with ASCII letters) currently cannot be implemented.
+ Most of the implemented algorithms have certain properties (like [commutative property](https://en.wikipedia.org/wiki/Commutative_property)) that make their behavior more like what you would expect and make normalization simple. So, I haven't implemented yet Needleman-Wunsch and Gotoh, mostly because they are tricky to normalize and I'm still not 100% sure that I did it correctly in the original textdistance.

## Acknowledgments

There are the libraries that I used as a reference implementation and the source of test cases:

+ 🐍 Python: [textdistance](https://github.com/life4/textdistance), [abydos](https://github.com/chrislit/abydos), [jellyfish](https://github.com/jamesturk/jellyfish).
+ ☕️ JS: [talisman](https://github.com/Yomguithereal/talisman).
+ 🦀 Rust: [strsim](https://github.com/dguo/strsim-rs), [distance](https://github.com/mbrlabs/distance), [levenshtein-rs](https://github.com/wooorm/levenshtein-rs).

Specials thanks to [Trevor Gross](https://github.com/tgross35) for transferring to me the ownership of the [textdistance](https://crates.io/crates/textdistance) name on crates.io.

## Testing locally

To run everything locally, all you need is Rust, Python, and [task](https://taskfile.dev/installation/). Execute `task all` to run all code formatters, linters, and tests.

Thank you ❤️
