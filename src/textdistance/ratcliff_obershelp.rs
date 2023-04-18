use super::algorithm::Algorithm;

pub struct RatcliffObershelp {}

impl RatcliffObershelp {
    fn from_str(&self, s1: &str, s2: &str) -> usize {
        self.from_iter(s1.chars(), s2.chars())
    }

    fn from_iter<C, E>(&self, s1: C, s2: C) -> usize
    where
        C: Iterator<Item = E>,
        E: Eq,
    {
        let s1: Vec<E> = s1.collect();
        let s2: Vec<E> = s2.collect();
        let s1_len = s1.len();
        let s2_len = s2.len();
        let mut stack = Vec::new();
        stack.push(((0, s1_len), (0, s2_len)));
        let mut result = 0;

        while !stack.is_empty() {
            let top = stack.pop().unwrap();
            let ((part1_start, part1_len), (part2_start, part2_len)) = top;
            let s1_part = s1[part1_start..(part1_start + part1_len)].iter();
            let s2_part: Vec<&E> = s2[part2_start..(part2_start + part2_len)].iter().collect();

            let mut dp = vec![vec![0; s2_len + 1]; s1_len + 1];
            let mut prefix1_end = 0;
            let mut prefix2_end = 0;
            let mut match_len: usize = 0;
            for (i1, c1) in s1_part.enumerate() {
                for (i2, c2) in s2_part.iter().enumerate() {
                    if &c1 == c2 {
                        let new_len: usize = dp[i1][i2] + 1;
                        dp[i1 + 1][i2 + 1] = new_len;
                        if new_len > match_len {
                            debug_assert!(i1 + 1 >= new_len);
                            debug_assert!(i2 + 1 >= new_len);
                            match_len = new_len;
                            prefix1_end = i1 + 1;
                            prefix2_end = i2 + 1;
                        };
                    }
                }
            }

            if match_len != 0 {
                let prefix1_len = prefix1_end - match_len;
                let prefix2_len = prefix2_end - match_len;
                if prefix1_len != 0 && prefix2_len != 0 {
                    stack.push(((part1_start, prefix1_len), (part2_start, prefix2_len)));
                }
                let suffix1_len = part1_len - prefix1_end;
                let suffix2_len = part2_len - prefix2_end;
                if suffix1_len != 0 && suffix2_len != 0 {
                    stack.push((
                        (part1_start + prefix1_end, suffix1_len),
                        (part2_start + prefix2_end, suffix2_len),
                    ));
                }
                result += match_len;
            }
        }

        2 * result
    }
}

impl Algorithm for RatcliffObershelp {
    fn similarity(&self, s1: &str, s2: &str) -> usize {
        self.from_str(s1, s2)
    }

    fn maximum(&self, s1: &str, s2: &str) -> usize {
        s1.len() + s2.len()
    }
}

const DEFAULT: RatcliffObershelp = RatcliffObershelp {};

pub fn ratcliff_obershelp(s1: &str, s2: &str) -> f64 {
    DEFAULT.normalized_distance(s1, s2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let f = ratcliff_obershelp;
        // assert_eq!(f("", ""), 0.0);
        assert_eq!(f("abc", ""), 1.);
        assert_eq!(f("", "abc"), 1.);
        assert_eq!(f("abc", "abc"), 0.);
        assert_eq!(
            DEFAULT.from_str("GESTALT PATTERN MATCHING", "GESTALT PRACTICE"),
            24
        );
        assert_eq!(
            DEFAULT.from_str("GESTALT PRACTICE", "GESTALT PATTERN MATCHING"),
            26
        );
    }
}
