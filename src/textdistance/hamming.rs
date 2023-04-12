pub fn hamming(s: &str, t: &str) -> usize {
    let mut result = 0;
    for (s_char, t_char) in s.chars().zip(t.chars()) {
        if s_char != t_char {
            result += 1
        }
    }
    let s_len = s.chars().count();
    let t_len = t.chars().count();
    result += s_len.abs_diff(t_len);
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(hamming("sitting", "sitting"), 0);
        assert_eq!(hamming("abcdefg", "hijklmn"), 7);
        assert_eq!(hamming("karolin", "kathrin"), 3);
        assert_eq!(hamming("hello", "world"), 4);
        assert_eq!(hamming("Rust", "rust"), 1);
        assert_eq!(hamming("hi mark", "hi markus"), 2);
    }
}
