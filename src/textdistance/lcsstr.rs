use super::algorithm::Algorithm;

pub struct LCSStr {}

impl LCSStr {
    fn calculate(&self, s1: &str, s2: &str) -> String {
        let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];
        let mut result_end = 0;
        let mut result_len = 0;
        for (i, c1) in s1.chars().enumerate() {
            for (j, c2) in s2.chars().enumerate() {
                if c1 == c2 {
                    let new_len = dp[i][j] + 1;
                    dp[i + 1][j + 1] = new_len;
                    if new_len > result_len {
                        result_len = new_len;
                        result_end = i + 1;
                    };
                }
            }
        }
        let result = s1.get((result_end - result_len)..result_end);
        result.unwrap().to_owned()
    }
}

impl Algorithm for LCSStr {
    fn similarity(&self, s1: &str, s2: &str) -> usize {
        self.calculate(s1, s2).len()
    }
}

const DEFAULT: LCSStr = LCSStr {};

pub fn lcsstr(s1: &str, s2: &str) -> String {
    DEFAULT.calculate(s1, s2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        let f = lcsstr;
        assert_eq!(f("", ""), "");
        assert_eq!(f("a", ""), "");
        assert_eq!(f("", "a"), "");
        assert_eq!(f("a", "a"), "a");
        assert_eq!(f("abcdef", "bcd"), "bcd");
        assert_eq!(f("bcd", "abcdef"), "bcd");
        assert_eq!(f("abcdef", "xabded"), "ab");
        assert_eq!(f("GeeksforGeeks", "GeeksQuiz"), "Geeks");
        assert_eq!(f("abcdxyz", "xyzabcd"), "abcd");
        assert_eq!(f("zxabcdezy", "yzabcdezx"), "abcdez");
        assert_eq!(
            f("OldSite:GeeksforGeeks.org", "NewSite:GeeksQuiz.com"),
            "Site:Geeks"
        );
    }
}
