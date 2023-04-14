use super::algorithm::Algorithm;

pub struct LCSStr {}

impl LCSStr {
    fn calculate(&self, s1: &str, s2: &str) -> usize {
        let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];
        let mut result = 0;
        for (i, c1) in s1.chars().enumerate() {
            for (j, c2) in s2.chars().enumerate() {
                if c1 == c2 {
                    dp[i + 1][j + 1] = 1 + dp[i][j];
                    result = result.max(dp[i + 1][j + 1]);
                }
            }
        }
        result
    }
}

impl Algorithm for LCSStr {
    fn similarity(&self, s1: &str, s2: &str) -> usize {
        self.calculate(s1, s2)
    }
}

const DEFAULT: LCSStr = LCSStr {};

pub fn lcsstr(s1: &str, s2: &str) -> usize {
    DEFAULT.calculate(s1, s2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        let f = lcsstr;
        assert_eq!(f("", ""), 0);
        assert_eq!(f("a", ""), 0);
        assert_eq!(f("", "a"), 0);
        assert_eq!(f("a", "a"), 1);
        assert_eq!(f("abcdef", "bcd"), 3);
        assert_eq!(f("bcd", "abcdef"), 3);
        assert_eq!(f("abcdef", "xabded"), 2);
        assert_eq!(f("GeeksforGeeks", "GeeksQuiz"), 5);
        assert_eq!(f("abcdxyz", "xyzabcd"), 4);
        assert_eq!(f("zxabcdezy", "yzabcdezx"), 6);
        assert_eq!(f("OldSite:GeeksforGeeks.org", "NewSite:GeeksQuiz.com"), 10);
    }
}
