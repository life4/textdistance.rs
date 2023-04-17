pub trait Algorithm {
    fn similarity(&self, s1: &str, s2: &str) -> usize {
        self.maximum(s1, s2) - self.distance(s1, s2)
    }

    fn distance(&self, s1: &str, s2: &str) -> usize {
        self.maximum(s1, s2) - self.similarity(s1, s2)
    }

    fn normalized_similarity(&self, s1: &str, s2: &str) -> f64 {
        let max = self.maximum(s1, s2);
        if max == 0 {
            1.
        } else {
            self.similarity(s1, s2) as f64 / max as f64
        }
    }

    fn normalized_distance(&self, s1: &str, s2: &str) -> f64 {
        let max = self.maximum(s1, s2);
        if max == 0 {
            0.
        } else {
            self.distance(s1, s2) as f64 / max as f64
        }
    }

    fn maximum(&self, s1: &str, s2: &str) -> usize {
        s1.chars().count().max(s2.chars().count())
    }
}
