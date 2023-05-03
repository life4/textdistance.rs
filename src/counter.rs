use std::collections::HashMap;
use std::hash::Hash;

/// Multiset container inspired by Python's `collections.Counter`.
pub struct Counter<K> {
    map: HashMap<K, usize>,
}

impl<K> Counter<K>
where
    K: Hash + Eq + Copy,
{
    /// make an empty Counter
    pub fn new() -> Counter<K> {
        Counter {
            map: HashMap::new(),
        }
    }

    /// Create a counter from a sequence.
    pub fn from_iter<I>(iter: I) -> Counter<K>
    where
        I: IntoIterator<Item = K>,
    {
        let mut counter = Counter::new();
        counter.update(iter);
        counter
    }

    /// Merge items from a sequence into the Counter
    pub fn update<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = K>,
    {
        for item in iter {
            let entry = self.map.entry(item).or_insert(0);
            *entry += 1;
        }
    }

    // How many items there are in total in the Counter.
    pub fn count(&self) -> usize {
        self.map.values().sum()
    }

    /// The count of things without the things
    pub fn values(&self) -> impl Iterator<Item = &usize> {
        self.map.values()
    }

    /// Merge two counters together.
    pub fn merge(&self, rhs: &Counter<K>) -> Counter<K> {
        let mut result: Counter<K> = Counter::new();
        result.map.extend(&self.map);
        result.map.extend(&rhs.map);
        result
    }

    /// How many there are common items in the given multisets.
    pub fn intersect_count(&self, rhs: &Counter<K>) -> usize {
        let mut result = 0;
        for (key, lhs_count) in &self.map {
            if let Some(rhs_count) = rhs.map.get(key) {
                result += lhs_count.min(rhs_count);
            }
        }
        result
    }

    /// How many there are items in total in both multisets.
    pub fn union_count(&self, rhs: &Counter<K>) -> usize {
        let mut result = 0;
        for (key, lhs_count) in &self.map {
            let rhs_count = rhs.map.get(key).unwrap_or(&0);
            result += lhs_count.max(rhs_count);
        }
        for (key, rhs_count) in &rhs.map {
            if self.map.get(key).is_none() {
                result += rhs_count;
            }
        }
        result
    }

    /// How many there are item in left that aren't in the right
    pub fn diff_count(&self, rhs: &Counter<K>) -> usize {
        let mut result = 0;
        for (key, lhs_count) in &self.map {
            let rhs_count = rhs.map.get(key).unwrap_or(&0);
            if lhs_count > rhs_count {
                result += lhs_count - rhs_count;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::assert;
    use rstest::rstest;

    pub fn eq<K: Hash + Eq + Copy>(lhs: &Counter<K>, rhs: &Counter<K>) -> bool {
        for (key, lhs_count) in &lhs.map {
            if let Some(rhs_count) = rhs.map.get(key) {
                if lhs_count != rhs_count {
                    return false;
                }
            } else {
                return false;
            }
        }
        for (key, rhs_count) in &rhs.map {
            if let Some(lhs_count) = lhs.map.get(key) {
                if lhs_count != rhs_count {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    #[rstest]
    fn smoke() {
        let c1 = Counter::from_iter(1..=5);
        let c2 = Counter::from_iter(3..=7);
        assert!(eq(&c1, &c1));
        assert!(!eq(&c1, &c2));
        // assert!(eq(c1.intersect(&c2), &Counter::from_iter(3..=5)));
        assert!(c1.intersect_count(&c2) == 3);
        assert!(c1.union_count(&c2) == 7);
    }
}
