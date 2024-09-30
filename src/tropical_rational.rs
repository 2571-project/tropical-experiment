use core::fmt;
use core::ops::{Add, Div, DivAssign, Mul, MulAssign};

use crate::tropical_int::TropicalInt;
use crate::tropical_polynomial::{Degree, TropicalPolynomial};

#[derive(PartialEq, Clone, Debug)]
pub struct TropicalRational<const N: usize> {
    pub(crate) numerator: TropicalPolynomial<N>,
    pub(crate) denominator: TropicalPolynomial<N>,
}

impl<const N: usize> TropicalRational<N> {
    pub fn new(numerator: TropicalPolynomial<N>, denominator: TropicalPolynomial<N>) -> Self {
        TropicalRational {
            numerator,
            denominator,
        }
    }

    pub fn polynomial(numerator: TropicalPolynomial<N>) -> Self {
        Self::new(
            numerator,
            TropicalPolynomial::from(vec![([0; N], TropicalInt::from(0))]),
        )
    }

    pub fn pow(&self, exponent: Degree) -> Self {
        TropicalRational::new(self.numerator.pow(exponent), self.denominator.pow(exponent))
    }

    // TODO: (hard)
    pub fn simplify(&self) -> Self {
        self.clone()
    }
}

impl<const N: usize> Add for TropicalRational<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let new_numerator = (self.numerator.clone() * rhs.denominator.clone())
            + (rhs.numerator.clone() * self.denominator.clone());
        let new_denominator = self.denominator * rhs.denominator;

        TropicalRational::new(new_numerator, new_denominator).simplify()
    }
}

impl<const N: usize> Mul for TropicalRational<N> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let new_numerator = self.numerator * other.numerator;
        let new_denominator = self.denominator * other.denominator;

        TropicalRational::new(new_numerator, new_denominator).simplify()
    }
}

impl<const N: usize> MulAssign for TropicalRational<N> {
    fn mul_assign(&mut self, other: Self) {
        *self = self.clone() * other;
    }
}

impl<const N: usize> Div for TropicalRational<N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        TropicalRational::new(
            self.numerator * rhs.denominator,
            self.denominator * rhs.numerator,
        )
    }
}

impl<const N: usize> DivAssign for TropicalRational<N> {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.clone() / rhs;
    }
}

impl<const N: usize> fmt::Display for TropicalRational<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}) / ({})", self.numerator, self.denominator)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        tropical_int::TropicalInt, tropical_polynomial::TropicalPolynomial,
        tropical_rational::TropicalRational,
    };

    #[test]
    fn test_tropical_rationals() {
        let p1: TropicalPolynomial<2> = TropicalPolynomial::from(vec![
            ([1, 0], TropicalInt::from(5)),
            ([0, 1], TropicalInt::from(3)),
        ]);

        let p2: TropicalPolynomial<2> = TropicalPolynomial::from(vec![
            ([1, 1], TropicalInt::from(2)),
            ([0, 0], TropicalInt::from(4)),
        ]);

        let r1 = TropicalRational::new(p1.clone(), p2.clone());
        let r2 = TropicalRational::new(p2, p1);

        println!("r1 = {}", r1);
        println!("r2 = {}", r2);
        println!("r1 + r2 = {}", r1.clone() + r2.clone());
        println!("r1 * r2 = {}", r1.clone() * r2.clone());
        println!("r1 / r2 = {}", r1 / r2);
    }
}
