use core::fmt;
use core::ops::{Add, AddAssign, Mul, MulAssign};
use std::collections::HashMap;
use std::iter::Sum;

use crate::tropical_int::TropicalInt;

// README: not ideal but I need to have inverse for automorphisms so...
pub type Degree = i64;

#[derive(Clone, Debug)]
pub struct TropicalPolynomial<const N: usize> {
    pub(crate) terms: HashMap<[Degree; N], TropicalInt>,
}

impl<const N: usize> TropicalPolynomial<N> {
    pub fn new() -> Self {
        TropicalPolynomial {
            terms: HashMap::new(),
        }
    }

    pub fn add_term(&mut self, multi_degree: [Degree; N], coefficient: TropicalInt) {
        let current_coefficient = self
            .terms
            .get(&multi_degree)
            .unwrap_or(&TropicalInt::AdditiveIdentity);

        if coefficient + *current_coefficient == coefficient {
            self.terms.insert(multi_degree, coefficient);
        }
    }

    pub fn get_term(&self, multi_degree: &[Degree; N]) -> Option<&TropicalInt> {
        self.terms.get(multi_degree)
    }

    pub fn monomial(multi_degree: [Degree; N], coefficent: TropicalInt) -> Self {
        Self::from(vec![(multi_degree, coefficent)])
    }

    pub fn variable(index: usize) -> Self {
        TropicalPolynomial::from(vec![(
            core::array::from_fn(|inner| if inner == index { 1 } else { 0 }),
            TropicalInt::from(0),
        )])
    }

    pub fn constant(constant: TropicalInt) -> Self {
        Self::monomial(core::array::from_fn(|_| 0), constant)
    }

    pub fn additive_identity() -> Self {
        Self::constant(TropicalInt::AdditiveIdentity)
    }

    pub fn multiplicative_identity() -> Self {
        Self::constant(TropicalInt::from(0))
    }

    pub fn pow(&self, power: Degree) -> Self {
        if power == 0 {
            return Self::multiplicative_identity();
        }

        // TODO: parallelize
        (1..=power).fold(Self::multiplicative_identity(), |acc, _| {
            acc.mul(self.clone())
        })

        // README: this is more efficient for evaluation
        // but not for formally computing the polynomial terms
        // Self::from(
        //     self.terms
        //         .iter()
        //         .map(|(multi_degree, coefficient)| {
        //             (
        //                 core::array::from_fn(|index| multi_degree[index] * power),
        //                 coefficient.pow(power),
        //             )
        //         })
        //         .collect::<Vec<([Degree; N], TropicalInt)>>(),
        // )
    }

    pub fn evaluate(&self, variables: [TropicalInt; N]) -> TropicalInt {
        self.terms.iter().fold(
            TropicalInt::AdditiveIdentity,
            |acc, (multi_degree, coefficient)| {
                acc + coefficient.clone()
                    * variables
                        .iter()
                        .zip(multi_degree.iter())
                        .fold(TropicalInt::zero(), |term, (variable, &degree)| {
                            term * variable.pow(degree)
                        })
            },
        )
    }
}

// TODO: implement for array and slices
impl<const N: usize> From<Vec<([Degree; N], TropicalInt)>> for TropicalPolynomial<N> {
    fn from(terms: Vec<([Degree; N], TropicalInt)>) -> Self {
        let mut result = HashMap::new();
        for term in terms {
            let current_coefficient = result
                .get(&term.0)
                .unwrap_or(&TropicalInt::AdditiveIdentity);

            if term.1 + *current_coefficient == term.1 {
                result.insert(term.0, term.1);
            }
        }

        Self { terms: result }
    }
}

impl<const N: usize> Add for TropicalPolynomial<N> {
    type Output = Self;

    // TODO: parallelize?
    fn add(self, rhs: Self) -> Self {
        let mut result = TropicalPolynomial::new();

        for (exponents, coefficient) in self.terms.iter().chain(rhs.terms.iter()) {
            let current_coefficient = result
                .terms
                .entry(*exponents)
                .or_insert_with(|| TropicalInt::from(0));
            *current_coefficient = current_coefficient.clone() + coefficient.clone();
        }

        result
    }
}

impl<const N: usize> AddAssign for TropicalPolynomial<N> {
    fn add_assign(&mut self, rhs: Self) {
        for (exponents, coefficient) in rhs.terms {
            let current_coefficient = self
                .terms
                .entry(exponents)
                .or_insert_with(|| TropicalInt::from(0));
            *current_coefficient = current_coefficient.clone() + coefficient;
        }
    }
}

impl<const N: usize> Mul for TropicalPolynomial<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut result = TropicalPolynomial::new();

        for (exponents1, coefficient1) in self.terms.iter() {
            for (exponents2, coefficient2) in rhs.terms.iter() {
                let mut new_exponents = [0; N];
                for i in 0..N {
                    new_exponents[i] = exponents1[i] + exponents2[i];
                }

                let coefficient_increment = coefficient1.clone() * coefficient2.clone();

                match coefficient_increment {
                    // README: it's probably slower to branch but we don't want to have terms in the map with coefficient equal to the additive identity, prefering to omit them.
                    TropicalInt::AdditiveIdentity => {}
                    TropicalInt::Integer(_) => {
                        let _ = result
                            .terms
                            .entry(new_exponents)
                            .and_modify(|current_coefficient| {
                                *current_coefficient += coefficient_increment
                            })
                            .or_insert_with(|| coefficient_increment);
                    }
                }
            }
        }

        result
    }
}

// TODO: in-place?
impl<const N: usize> MulAssign for TropicalPolynomial<N> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}

// TODO: iterate only once? not sure would be faster and also I don't think we need to
// optimize this
impl<const N: usize> PartialEq for TropicalPolynomial<N> {
    fn eq(&self, other: &Self) -> bool {
        for (exponents1, coefficient1) in self.terms.iter() {
            if let Some(coefficient2) = other.terms.get(exponents1) {
                if coefficient1 != coefficient2 {
                    return false;
                }
            } else {
                return false;
            }
        }

        for (exponents1, coefficient1) in other.terms.iter() {
            if let Some(coefficient2) = self.terms.get(exponents1) {
                if coefficient1 != coefficient2 {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}

const VARIABLES: &[char] = &['x', 'y', 'z'];
impl<const N: usize> fmt::Display for TropicalPolynomial<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut terms: Vec<_> = self.terms.iter().collect();
        terms.sort_by(|a, b| b.0.cmp(a.0)); // Sort in descending order of exponents

        for (i, (exponents, coefficient)) in terms.iter().enumerate() {
            if i > 0 {
                write!(f, " + ")?;
            }
            write!(f, "{}", coefficient)?;

            if N > 3 {
                for (j, &exp) in exponents.iter().enumerate() {
                    write!(f, "x{}^{}", j, exp)?;
                }
            } else {
                for (j, &exp) in exponents.iter().enumerate() {
                    write!(f, "{}^{}", VARIABLES[j], exp)?;
                }
            }
        }

        Ok(())
    }
}

impl<const N: usize> Sum for TropicalPolynomial<N> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(TropicalPolynomial::additive_identity(), |a, b| a + b)
    }
}

#[cfg(test)]
mod tests {
    use crate::{tropical_int::TropicalInt, tropical_polynomial::TropicalPolynomial};

    use super::Degree;

    #[test]
    fn test_sum_disjoint_polys() {
        let p1_terms: Vec<([Degree; 3], TropicalInt)> = vec![
            ([1, 0, 2], TropicalInt::from(5)),
            ([0, 1, 1], TropicalInt::from(3)),
        ];
        let p1 = TropicalPolynomial::from(p1_terms.clone());

        let p2_terms: Vec<([Degree; 3], TropicalInt)> = vec![
            ([1, 1, 0], TropicalInt::from(2)),
            ([0, 0, 1], TropicalInt::from(4)),
        ];
        let p2 = TropicalPolynomial::from(p2_terms.clone());

        let mut p3: TropicalPolynomial<3> = TropicalPolynomial::from(p1_terms);
        for (exponents, coefficient) in p2_terms {
            p3.add_term(exponents, coefficient);
        }

        assert_eq!(p1 + p2, p3);
    }

    #[test]
    fn test_sum_non_disjoint_polys() {
        let a = TropicalPolynomial::from(vec![([1, 1, 1], TropicalInt::from(5))]);

        let b = TropicalPolynomial::from(vec![([1, 1, 1], TropicalInt::from(7))]);

        assert_eq!(a + b.clone(), b);
    }

    #[test]
    fn test_poly_mul_2() {
        let test_table: Vec<[TropicalPolynomial<2>; 3]> = vec![[
            // README: this test case is subtle and it concerns how we deal with the absorptions of multiplication by the additive identity, not sure this is the best way to deal with this but it ensures we don't introduce ill-definedness to our representation
            TropicalPolynomial::monomial([2, 0], TropicalInt::from(0)),
            TropicalPolynomial::constant(TropicalInt::AdditiveIdentity),
            TropicalPolynomial::new(),
        ]];

        for [a, b, c] in test_table {
            assert_eq!(a * b, c);
        }
    }

    #[test]
    fn test_poly_mul_3() {
        let test_table: Vec<[TropicalPolynomial<3>; 3]> = vec![
            [
                TropicalPolynomial::from(vec![([1, 1, 0], TropicalInt::from(5))]),
                TropicalPolynomial::from(vec![([1, 0, 1], TropicalInt::from(7))]),
                TropicalPolynomial::from(vec![([2, 1, 1], TropicalInt::from(12))]),
            ],
            [
                TropicalPolynomial::from(vec![([1, 1, 0], TropicalInt::from(5))]),
                TropicalPolynomial::from(vec![
                    ([1, 0, 1], TropicalInt::from(7)),
                    ([4, 2, 1], TropicalInt::from(7)),
                ]),
                TropicalPolynomial::from(vec![
                    ([2, 1, 1], TropicalInt::from(12)),
                    ([5, 3, 1], TropicalInt::from(12)),
                ]),
            ],
            [
                TropicalPolynomial::from(vec![
                    ([1, 1, 0], TropicalInt::from(5)),
                    ([0, 6, 2], TropicalInt::from(-3)),
                ]),
                TropicalPolynomial::from(vec![
                    ([1, 0, 1], TropicalInt::from(7)),
                    ([4, 2, 1], TropicalInt::from(7)),
                ]),
                TropicalPolynomial::from(vec![
                    ([2, 1, 1], TropicalInt::from(12)),
                    ([5, 3, 1], TropicalInt::from(12)),
                    ([1, 6, 3], TropicalInt::from(4)),
                    ([4, 8, 3], TropicalInt::from(4)),
                ]),
            ],
        ];

        for [a, b, c] in test_table {
            assert_eq!(a * b, c);
        }
    }

    #[test]
    fn test_poly_pow() {
        let test_table: Vec<(TropicalPolynomial<3>, Degree, TropicalPolynomial<3>)> = vec![
            (
                TropicalPolynomial::multiplicative_identity(),
                3,
                TropicalPolynomial::multiplicative_identity(),
            ),
            (
                TropicalPolynomial::variable(0),
                4,
                TropicalPolynomial::from(vec![([4, 0, 0], TropicalInt::from(0))]),
            ),
            (
                TropicalPolynomial::variable(0) + TropicalPolynomial::variable(1),
                4,
                TropicalPolynomial::from(vec![
                    ([4, 0, 0], TropicalInt::from(0)),
                    ([0, 4, 0], TropicalInt::from(0)),
                ]),
            ),
            (
                TropicalPolynomial::from(vec![
                    ([4, 0, 3], TropicalInt::from(5)),
                    ([0, 4, 1], TropicalInt::from(7)),
                ]),
                2,
                TropicalPolynomial::from(vec![
                    ([8, 0, 6], TropicalInt::from(10)),
                    ([0, 8, 2], TropicalInt::from(14)),
                ]),
            ),
        ];

        for (a, power, b) in test_table {
            assert_eq!(a.pow(power), b);
        }
    }

    #[test]
    fn test_evaluate() {
        let test_table: Vec<(TropicalPolynomial<3>, [TropicalInt; 3], TropicalInt)> = vec![
            (
                TropicalPolynomial::monomial([1, 2, 3], TropicalInt::from(5)),
                [
                    TropicalInt::from(4),
                    TropicalInt::from(2),
                    TropicalInt::from(1),
                ],
                TropicalInt::from(16),
            ),
            (
                TropicalPolynomial::from(vec![
                    ([1, 2, 3], TropicalInt::from(5)),
                    ([0, 5, 17], TropicalInt::from(2)),
                ]),
                [
                    TropicalInt::from(4),
                    TropicalInt::from(2),
                    TropicalInt::from(1),
                ],
                TropicalInt::from(29),
            ),
        ];

        for (poly, vars, result) in test_table {
            assert_eq!(poly.evaluate(vars), result);
        }
    }

    #[test]
    fn test_simplify_dominated_terms() {
        let test_table: Vec<(TropicalPolynomial<2>, TropicalPolynomial<2>)> = vec![
            (
                TropicalPolynomial::from(vec![
                    ([2, 3], TropicalInt::Integer(5)),
                    ([2, 3], TropicalInt::Integer(4)),
                ]),
                TropicalPolynomial::from(vec![([2, 3], TropicalInt::Integer(5))]),
            ),
            (
                TropicalPolynomial::from(vec![
                    ([2, 2], TropicalInt::Integer(6)),
                    ([3, 2], TropicalInt::Integer(7)),
                    ([2, 3], TropicalInt::Integer(4)),
                    ([2, 3], TropicalInt::Integer(5)),
                ]),
                TropicalPolynomial::from(vec![
                    ([2, 2], TropicalInt::Integer(6)),
                    ([3, 2], TropicalInt::Integer(7)),
                    ([2, 3], TropicalInt::Integer(5)),
                ]),
            ),
        ];

        for (p, q) in test_table {
            println!("p = {p} | q = {q}");
            assert_eq!(p, q);
        }
    }
}
