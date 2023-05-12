use std::cmp;
use std::ops;

#[derive(Clone, Copy)]
pub struct Fraction(pub usize, pub usize);

impl Fraction {
    pub fn as_f64(&self) -> f64 {
        self.0 as f64 / self.1 as f64
    }
}

impl ops::Mul<&Fraction> for &Fraction {
    type Output = Fraction;

    fn mul(self, rhs: &Fraction) -> Fraction {
        Fraction(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl ops::AddAssign<Fraction> for Fraction {
    fn add_assign(&mut self, rhs: Fraction) {
        if self.1 == rhs.1 {
            self.0 += rhs.0;
        } else {
            self.0 = self.0 * rhs.1 + rhs.0 * self.1;
            self.1 *= rhs.1;
        }
    }
}

impl ops::Add<Fraction> for Fraction {
    type Output = Fraction;

    fn add(self, rhs: Fraction) -> Fraction {
        Fraction(self.0 * rhs.1 + rhs.0 * self.1, self.1 * rhs.1)
    }
}

impl ops::MulAssign<&Fraction> for Fraction {
    fn mul_assign(&mut self, rhs: &Fraction) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
    }
}

impl cmp::PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        let left = self.0 as f64 / self.1 as f64;
        let right = other.0 as f64 / other.1 as f64;
        left == right
    }
}

impl cmp::PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        let left = self.0 as f64 / self.1 as f64;
        let right = other.0 as f64 / other.1 as f64;
        left.partial_cmp(&right)
    }
}
