use ndarray::Array2;

const GAP_COST: i64 = 1;

pub fn smith_waterman(s1: &str, s2: &str) -> i64 {
    let s1_len = s1.chars().count();
    let s2_len = s2.chars().count();
    let mut dist_mat = Array2::from_elem((s1_len + 1, s2_len + 1), 0);
    for (i, sc1) in s1.chars().enumerate() {
        for (j, sc2) in s1.chars().enumerate() {
            let shift = if sc1 == sc2 { 1 } else { 0 };
            let match_ = dist_mat[[i, j]] + shift;
            let delete = dist_mat[[i, j + 1]] - GAP_COST;
            let insert = dist_mat[[i + 1, j]] - GAP_COST;
            dist_mat[[i + 1, j + 1]] = 0.max(match_).max(delete).max(insert)
        }
    }
    dist_mat[[s1_len, s2_len]]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let f = smith_waterman;
        assert_eq!(f("abcd", "abcd"), 4);
        assert_eq!(f("abcd", "efgh"), 4);
    }
}
