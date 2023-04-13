use std::ops::{Div, Sub};
pub trait Algorithm<R>
where
    R: Sub<Output = R> + Div<Output = R>,
{
    fn similarity(&self, s1: &str, s2: &str) -> R {
        self.maximum(s1, s2) - self.distance(s1, s2)
    }

    fn distance(&self, s1: &str, s2: &str) -> R {
        self.maximum(s1, s2) - self.similarity(s1, s2)
    }

    fn normalized_similarity(&self, s1: &str, s2: &str) -> R {
        self.similarity(s1, s2) / self.maximum(s1, s2)
    }

    fn normalized_distance(&self, s1: &str, s2: &str) -> R {
        self.distance(s1, s2) / self.maximum(s1, s2)
    }

    fn maximum(&self, s1: &str, s2: &str) -> R;
}
