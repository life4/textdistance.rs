use super::algorithm::Algorithm;
use ndarray::Array2;

struct LCSSeq {}

impl LCSSeq {
    fn calculate(&self, s1: &str, s2: &str) -> String {
        let s1_len = s1.chars().count();
        let s2_len = s2.chars().count();
        let mut lengths = Array2::from_elem((s1_len + 1, s2_len + 1), 0);

        for (i, char1) in s1.chars().enumerate() {
            for (j, char2) in s2.chars().enumerate() {
                lengths[[i + 1, j + 1]] = if char1 == char2 {
                    lengths[[i, j]] + 1
                } else {
                    lengths[[i + 1, j]].max(lengths[[i, j + 1]])
                };
            }
        }

        let mut result = String::new();
        let mut i = s1_len;
        let mut j = s2_len;
        while i != 0 && j != 0 {
            if lengths[[i, j]] == lengths[[i - 1, j]] {
                i -= 1;
            } else if lengths[[i, j]] == lengths[[i, j - 1]] {
                j -= 1;
            } else {
                // assert s1[i - 1] == s2[j - 1]
                result.push(s1.chars().nth(i - 1).unwrap());
                i -= 1;
                j -= 1;
            }
        }
        result.chars().rev().collect::<String>()
    }
}

impl Algorithm<usize> for LCSSeq {
    fn similarity(&self, s1: &str, s2: &str) -> usize {
        self.calculate(s1, s2).chars().count()
    }

    fn maximum(&self, s1: &str, s2: &str) -> usize {
        s1.chars().count().max(s2.chars().count())
    }
}

const DEFAULT: LCSSeq = LCSSeq {};

pub fn lcsseq(s1: &str, s2: &str) -> String {
    DEFAULT.calculate(s1, s2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let f = lcsseq;
        assert_eq!(f("ab", "cd"), "");
        assert_eq!(f("abcd", "abcd"), "abcd");
        assert_eq!(f("test", "text"), "tet");
        assert_eq!(f("thisisatest", "testing123testing"), "tsitest");
    }
}
