pub trait Algorithm {
    fn from_iter<C, E>(&self, s1: C, s2: C) -> Result
    where
        C: Iterator<Item = E>,
        E: Eq + Copy;

    fn from_str(&self, s1: &str, s2: &str) -> Result {
        self.from_iter(s1.chars(), s2.chars())
    }

    fn similarity<C, E>(&self, s1: C, s2: C) -> usize
    where
        C: Iterator<Item = E>,
        E: Eq + Copy,
    {
        self.from_iter(s1, s2).similarity()
    }

    fn distance<C, E>(&self, s1: C, s2: C) -> usize
    where
        C: Iterator<Item = E>,
        E: Eq + Copy,
    {
        self.from_iter(s1, s2).distance()
    }

    fn normalized_similarity<C, E>(&self, s1: C, s2: C) -> f64
    where
        C: Iterator<Item = E>,
        E: Eq + Copy,
    {
        self.from_iter(s1, s2).normalized_similarity()
    }

    fn normalized_distance<C, E>(&self, s1: C, s2: C) -> f64
    where
        C: Iterator<Item = E>,
        E: Eq + Copy,
    {
        self.from_iter(s1, s2).normalized_distance()
    }
}

pub struct Result {
    pub is_distance: bool,
    pub abs: usize,
    pub max: usize,
    pub len1: usize,
    pub len2: usize,
}

impl Result {
    pub fn distance(&self) -> usize {
        if self.is_distance {
            self.abs
        } else {
            self.max - self.abs
        }
    }

    pub fn similarity(&self) -> usize {
        if self.is_distance {
            self.max - self.abs
        } else {
            self.abs
        }
    }

    pub fn normalized_distance(&self) -> f64 {
        self.distance() as f64 / self.max as f64
    }

    pub fn normalized_similarity(&self) -> f64 {
        self.similarity() as f64 / self.max as f64
    }
}
