use super::algorithm::Algorithm;

pub struct LCSStr {}

impl LCSStr {
    fn from_str(&self, s1: &str, s2: &str) -> String {
        let res = self.from_iter(s1.chars(), s2.chars());
        res.into_iter().collect()
    }

    fn from_iter<C, E>(&self, s1: C, s2: C) -> Vec<E>
    where
        C: Iterator<Item = E> + Clone,
        E: Eq,
    {
        let mut dp = vec![vec![0; s2.to_owned().count() + 1]; s1.to_owned().count() + 1];
        let mut result_end = 0;
        let mut result_len = 0;
        for (i, c1) in s1.to_owned().enumerate() {
            for (j, c2) in s2.to_owned().enumerate() {
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
        s1.skip(result_end - result_len).take(result_len).collect()
    }
}

impl Algorithm for LCSStr {
    fn similarity(&self, s1: &str, s2: &str) -> usize {
        self.from_str(s1, s2).len()
    }
}

const DEFAULT: LCSStr = LCSStr {};

pub fn lcsstr(s1: &str, s2: &str) -> String {
    DEFAULT.from_str(s1, s2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn basic() {
        let f = lcsstr;
        assert_eq!(f("", ""), "");
        assert_eq!(f("a", ""), "");
        assert_eq!(f("", "a"), "");
        assert_eq!(f("a", "a"), "a");
        assert_eq!(f("ab", "b"), "b");
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

    #[test]
    fn unicode() {
        let f = lcsstr;
        assert_eq!(f("п", ""), "");
        assert_eq!(f("", "п"), "");
        assert_eq!(f("п", "п"), "п");
        assert_eq!(f("привет", "пока"), "п");
        assert_eq!(f("корвет", "привет"), "вет");
    }

    proptest! {
        #[test]
        fn prop(s1 in ".*", s2 in ".*") {
            let res = lcsstr(&s1, &s2);
            prop_assert!(s1.contains(&res));
            prop_assert!(s2.contains(&res));
        }
    }
}
