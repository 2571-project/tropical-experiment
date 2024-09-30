use core::cmp::Ordering;
use core::fmt;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign};

use num_bigint::BigInt;

#[derive(Clone, Debug)]
pub enum TropicalInt {
    AdditiveIdentity,
    Integer(BigInt),
}

impl TropicalInt {
    pub fn new(value: BigInt) -> Self {
        Self::Integer(value)
    }

    pub fn zero() -> Self {
        Self::from(0)
    }

    pub fn multiplicative_identity() -> Self {
        Self::zero()
    }

    pub fn pow<D>(&self, power: D) -> Self
    where
        D: Into<BigInt>,
    {
        match self {
            Self::Integer(int) => Self::new(int.clone() * power.into()),
            x => x.clone(),
        }
    }
}

impl PartialEq for TropicalInt {
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (Self::Integer(a), Self::Integer(b)) => a == b,
            (Self::AdditiveIdentity, Self::AdditiveIdentity) => true,
            _ => false,
        }
    }
}

impl Eq for TropicalInt {}

impl PartialOrd for TropicalInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TropicalInt {
    fn cmp(&self, rhs: &Self) -> Ordering {
        match (self, rhs) {
            (Self::Integer(a), Self::Integer(b)) => a.cmp(b),
            (Self::AdditiveIdentity, Self::AdditiveIdentity) => Ordering::Equal,
            (Self::AdditiveIdentity, _) => Ordering::Less,
            (_, Self::AdditiveIdentity) => Ordering::Greater,
        }
    }
}

impl Add for TropicalInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Integer(a), Self::Integer(b)) => Self::Integer(a.max(b)),
            (Self::AdditiveIdentity, b) => b,
            (a, Self::AdditiveIdentity) => a,
        }
    }
}

impl AddAssign for TropicalInt {
    fn add_assign(&mut self, rhs: Self) {
        // TODO: can we avoid cloning?
        *self = self.clone() + rhs;
    }
}

impl Mul for TropicalInt {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        match (self, rhs) {
            #[allow(clippy::suspicious_arithmetic_impl)]
            (Self::Integer(a), Self::Integer(b)) => Self::Integer(a + b),
            _ => Self::AdditiveIdentity,
        }
    }
}

impl MulAssign for TropicalInt {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}

impl Div for TropicalInt {
    type Output = TropicalInt;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            #[allow(clippy::suspicious_arithmetic_impl)]
            (Self::Integer(a), Self::Integer(b)) => Self::Integer(a - b),
            // FIXME: change Output to Result instead?
            (_, Self::AdditiveIdentity) => panic!("div by -inf"),
            (Self::AdditiveIdentity, _) => Self::AdditiveIdentity,
        }
    }
}

impl DivAssign for TropicalInt {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.clone() / rhs;
    }
}

impl fmt::Display for TropicalInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Integer(a) => write!(f, "{a}"),
            Self::AdditiveIdentity => write!(f, "-∞"),
        }
    }
}

impl From<i64> for TropicalInt {
    fn from(value: i64) -> Self {
        TropicalInt::new(BigInt::from(value))
    }
}

#[cfg(test)]
mod tests {
    use super::TropicalInt;

    #[test]
    fn test_tropical_plus() {
        let test_table: Vec<[TropicalInt; 3]> = vec![
            [
                TropicalInt::from(3),
                TropicalInt::from(5),
                TropicalInt::from(5),
            ],
            [
                TropicalInt::from(-1),
                TropicalInt::from(-2),
                TropicalInt::from(-1),
            ],
        ];

        for [a, b, c] in test_table {
            assert_eq!(a + b, c);
        }
    }
}
