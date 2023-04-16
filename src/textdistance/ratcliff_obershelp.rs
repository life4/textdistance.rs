use super::algorithm::Algorithm;

pub struct RatcliffObershelp {}

impl RatcliffObershelp {
    fn from_str(&self, s1: &str, s2: &str) -> usize {
        self.from_iterator(s1.chars(), s2.chars())
    }

    fn from_iterator<C, E>(&self, s1: C, s2: C) -> usize
    where
        C: Iterator<Item = E> + Clone,
        E: Eq,
    {
        let res = self.find(s1.to_owned(), s2.to_owned());
        2 * res
    }

    fn find<C, E>(&self, s1: C, s2: C) -> usize
    where
        C: Iterator<Item = E> + Clone,
        E: Eq,
    {
        let s1_len = s1.to_owned().count();
        let s2_len = s2.to_owned().count();
        let mut stack = Vec::new();
        stack.push(((0, s1_len), (0, s2_len)));
        let mut result = 0;

        while !stack.is_empty() {
            let top = stack.pop().unwrap();
            let ((part1_start, part1_len), (part2_start, part2_len)) = top;
            let s1_part = s1.to_owned().skip(part1_start).take(part1_len);
            let s2_part = s2.to_owned().skip(part2_start).take(part2_len);

            let mut dp = vec![vec![0; s2_len + 1]; s1_len + 1];
            let mut prefix1_end = 0;
            let mut prefix2_end = 0;
            let mut match_len: usize = 0;
            for (i, c1) in s1_part.enumerate() {
                for (j, c2) in s2_part.to_owned().enumerate() {
                    if c1 == c2 {
                        let new_len: usize = dp[i][j] + 1;
                        dp[i + 1][j + 1] = new_len;
                        if new_len > match_len {
                            debug_assert!(i + 1 >= new_len);
                            debug_assert!(j + 1 >= new_len);
                            match_len = new_len;
                            prefix1_end = i + 1;
                            prefix2_end = j + 1;
                        };
                    }
                }
            }

            if match_len != 0 {
                let prefix1_len = prefix1_end - match_len;
                let prefix2_len = prefix2_end - match_len;
                stack.push(((part1_start, prefix1_len), (part2_start, prefix2_len)));
                stack.push((
                    (part1_start + prefix1_end, part1_len - prefix1_end),
                    (part2_start + prefix2_end, part2_len - prefix2_end),
                ));
                result += match_len;
            }
        }

        result
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
    use proptest::prelude::*;

    #[test]
    fn basic() {
        // let f = ratcliff_obershelp;
        // assert_eq!(f("", ""), 0.0);
        assert_eq!(
            DEFAULT.from_str("GESTALT PATTERN MATCHING", "GESTALT PRACTICE"),
            24
        );
        assert_eq!(
            DEFAULT.from_str("GESTALT PRACTICE", "GESTALT PATTERN MATCHING"),
            26
        );
    }

    proptest! {
        #[test]
        fn prop(s1 in ".+", s2 in ".*") {
            let res = ratcliff_obershelp(&s1, &s2);
            prop_assert!(0. <= res );
            prop_assert!(res <= 1.);
        }
    }
}
