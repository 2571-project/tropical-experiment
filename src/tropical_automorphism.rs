use core::fmt;

use crate::{
    tropical_int::TropicalInt,
    tropical_polynomial::{Degree, TropicalPolynomial},
    tropical_rational::TropicalRational,
};

#[derive(PartialEq, Clone, Debug)]
pub struct TropicalAutomorphism<const N: usize> {
    mappings: [TropicalRational<N>; N],
    // TODO: inverse: [TropicalRational<N>; N],
}

impl<const N: usize> TropicalAutomorphism<N> {
    pub fn new(mappings: [TropicalRational<N>; N]) -> Self {
        Self { mappings }
    }

    pub fn identity() -> Self {
        Self {
            mappings: core::array::from_fn(|index| {
                TropicalRational::polynomial(TropicalPolynomial::variable(index))
            }),
        }
    }

    pub fn scalar(factor: TropicalInt) -> Self {
        Self {
            mappings: core::array::from_fn(|index| {
                TropicalRational::polynomial(
                    TropicalPolynomial::variable(index)
                        * TropicalPolynomial::constant(factor.clone()),
                )
            }),
        }
    }

    pub fn monomial(degrees_matrix: [[Degree; N]; N], coefficients: [TropicalInt; N]) -> Self {
        debug_assert!(
            degrees_matrix.iter().all(|row| row.iter().all(|d| *d > 0)),
            "monomials must have positive degrees"
        );

        // TODO: assert degrees_matrix has determinant != 0
        Self {
            mappings: core::array::from_fn(|index| {
                TropicalRational::polynomial(TropicalPolynomial::from(vec![(
                    degrees_matrix[index],
                    coefficients[index].clone(),
                )]))
            }),
        }
    }

    pub fn elementary_triangular(variable: usize, row: TropicalPolynomial<N>) -> Self {
        debug_assert!(variable < N);
        debug_assert!(row
            .terms
            .iter()
            .all(|(multi_degree, _)| { multi_degree.iter().take(variable + 1).all(|d| *d == 0) }));

        let mut mappings = core::array::from_fn(|index| {
            TropicalRational::polynomial(TropicalPolynomial::variable(index))
        });

        mappings[variable] *= TropicalRational::polynomial(row);

        Self { mappings }
    }

    pub fn inverse_elementary_triangular(variable: usize, row: TropicalPolynomial<N>) -> Self {
        assert!(variable < N);
        assert!(row.terms.iter().all(|(multi_degree, _)| {
            multi_degree.iter().take(variable).all(|d| *d == 0) && multi_degree[variable] == 0
        }));

        let mut mappings = core::array::from_fn(|index| {
            TropicalRational::polynomial(TropicalPolynomial::variable(index))
        });

        mappings[variable] /= TropicalRational::polynomial(row);

        Self { mappings }
    }

    pub fn apply(&self, poly: &TropicalPolynomial<N>) -> TropicalPolynomial<N> {
        poly.terms.iter().fold(
            TropicalPolynomial::new(),
            |acc, (multi_degree, coefficient)| {
                acc + multi_degree.iter().enumerate().fold(
                    TropicalPolynomial::constant(coefficient.clone()),
                    |term, (index, degree)| term * self.mappings[index].numerator.pow(*degree),
                )
            },
        )
    }

    pub fn compose(&self, rhs: Self) -> Self {
        Self {
            mappings: self
                .mappings
                .clone()
                .map(|lhs| TropicalRational::new(rhs.apply(&lhs.numerator), lhs.denominator)),
        }
    }
}

impl TropicalAutomorphism<2> {
    pub fn inverse_monomial(
        degrees_matrix: [[Degree; 2]; 2],
        coefficients: [TropicalInt; 2],
    ) -> Self {
        let [[a, b], [c, d]] = degrees_matrix;
        let det = (a * b) - (c * d);
        debug_assert!(det != 0,
            "degrees matrix needs to be invertible in order to calculate the monomial automorphism inverse"
        );
        debug_assert!(
            det == 1 || det == -1,
            "degrees matrix needs to have determinant 1 or -1 in order for the inverse's degrees matrix entries to be integers"
        );

        let degrees_matrix_inv = [[det * d, det * -b], [det * -c, det * a]];

        Self::monomial(
            degrees_matrix_inv,
            [coefficients[0].pow(-1), coefficients[1].pow(-1)],
        )
    }
}

impl<const N: usize> fmt::Display for TropicalAutomorphism<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        for (i, rational) in self.mappings.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", rational)?;
        }
        write!(f, ")")
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        tropical_int::TropicalInt, tropical_polynomial::TropicalPolynomial,
        tropical_rational::TropicalRational,
    };

    use super::TropicalAutomorphism;

    #[test]
    fn test_apply_2() {
        let test_table: Vec<(
            TropicalAutomorphism<2>,
            TropicalPolynomial<2>,
            TropicalPolynomial<2>,
        )> = vec![
            (
                /*
                    a(x, y) = (x³y^4, xy²) | a: T² -> T²
                    u(x, y) = x² + y² | u: T² ->
                    (u o a) = x^6y^8 + x^2y^4
                */
                TropicalAutomorphism::new([
                    TropicalRational::polynomial(TropicalPolynomial::monomial(
                        [3, 4],
                        TropicalInt::zero(),
                    )),
                    TropicalRational::polynomial(TropicalPolynomial::monomial(
                        [1, 2],
                        TropicalInt::zero(),
                    )),
                ]),
                TropicalPolynomial::from(vec![
                    ([2, 0], TropicalInt::zero()),
                    ([0, 2], TropicalInt::zero()),
                ]),
                TropicalPolynomial::from(vec![
                    ([2, 4], TropicalInt::zero()),
                    ([6, 8], TropicalInt::zero()),
                ]),
            ),
            (
                /*
                    a(x, y) = (x³y^4, xy²) | a: T² -> T²
                    u(x, y) = 1 + xy | u: T² -> T
                    (u o a) = 1 + x^4y^6
                */
                TropicalAutomorphism::new([
                    TropicalRational::polynomial(TropicalPolynomial::monomial(
                        [3, 4],
                        TropicalInt::zero(),
                    )),
                    TropicalRational::polynomial(TropicalPolynomial::monomial(
                        [1, 2],
                        TropicalInt::zero(),
                    )),
                ]),
                TropicalPolynomial::from(vec![
                    ([0, 0], TropicalInt::from(1)),
                    ([1, 1], TropicalInt::from(1)),
                ]),
                TropicalPolynomial::from(vec![
                    ([0, 0], TropicalInt::from(1)),
                    ([4, 6], TropicalInt::from(1)),
                ]),
            ),
            (
                /*
                  a(x, y) = (1xy², 2x²y)
                  u(x, y) = 2xy³
                  (u o a) = 2(1xy²)(2x²y)³ = 9x^7y^5
                */
                TropicalAutomorphism::monomial(
                    [[1, 2], [2, 1]],
                    [TropicalInt::from(1), TropicalInt::from(2)],
                ),
                TropicalPolynomial::monomial([1, 3], TropicalInt::from(2)),
                TropicalPolynomial::monomial([7, 5], TropicalInt::from(9)),
            ),
        ];

        for (alfa, u, v) in test_table {
            assert_eq!(alfa.apply(&u), v);
        }
    }

    #[test]
    fn test_apply_3() {
        let test_table: Vec<(
            TropicalAutomorphism<3>,
            TropicalPolynomial<3>,
            TropicalPolynomial<3>,
        )> = vec![
            (
                TropicalAutomorphism::scalar(TropicalInt::from(2)),
                TropicalPolynomial::monomial([1, 2, 3], TropicalInt::from(6)),
                TropicalPolynomial::monomial([1, 2, 3], TropicalInt::from(18)),
            ),
            /*
               alfa = {
                   x -> 1xy²z^4
                   y -> 2xy³z^9
                   z -> 3xy^5z^25
               }
               u = 6xy²z³

               alfa(u) = 6(1xy²z^4)(2xy³z^9)²(3xy^5z^25)³
                       = (6.1.4.9)(x.x².x³)(y².y^6.y^15)(z^4.z^18.z^75)
                       = 20x^6.y^23.z^97
            */
            (
                TropicalAutomorphism::monomial(
                    [[1, 2, 4], [1, 3, 9], [1, 5, 25]],
                    [
                        TropicalInt::from(1),
                        TropicalInt::from(2),
                        TropicalInt::from(3),
                    ],
                ),
                TropicalPolynomial::monomial([1, 2, 3], TropicalInt::from(6)),
                TropicalPolynomial::monomial([6, 23, 97], TropicalInt::from(20)),
            ),
            (
                TropicalAutomorphism::monomial(
                    [[1, 2, 4], [1, 3, 9], [1, 5, 25]],
                    [
                        TropicalInt::from(1),
                        TropicalInt::from(2),
                        TropicalInt::from(3),
                    ],
                ),
                TropicalPolynomial::monomial([1, 2, 3], TropicalInt::from(6)),
                TropicalPolynomial::monomial([6, 23, 97], TropicalInt::from(20)),
            ),
        ];

        for (alfa, u, v) in test_table {
            assert_eq!(alfa.apply(&u), v);
        }
    }

    #[test]
    fn test_compose_2() {
        let test_table: Vec<[TropicalAutomorphism<2>; 3]> = vec![
            [
                /*
                   a = { x -> 1xy² | y -> y }
                   b = { x -> x | y -> y }
                   a o b = a
                */
                TropicalAutomorphism::elementary_triangular(
                    0,
                    TropicalPolynomial::monomial([0, 3], TropicalInt::from(2)),
                ),
                TropicalAutomorphism::elementary_triangular(
                    1,
                    TropicalPolynomial::constant(TropicalInt::from(0)),
                ),
                TropicalAutomorphism::new([
                    TropicalRational::polynomial(TropicalPolynomial::monomial(
                        [1, 3],
                        TropicalInt::from(2),
                    )),
                    TropicalRational::polynomial(TropicalPolynomial::variable(1)),
                ]),
            ],
            [
                /*
                    a(x, y) = (1xy², 2x²y)
                    b(x, y) = (2xy³, y)
                    (a o b) = (
                        1(2xy³)y² = 3xy^5,
                        2(2xy³)²y = 6x²y^7
                    )
                */
                TropicalAutomorphism::monomial(
                    [[1, 2], [2, 1]],
                    [TropicalInt::from(1), TropicalInt::from(2)],
                ),
                TropicalAutomorphism::elementary_triangular(
                    0,
                    TropicalPolynomial::monomial([0, 3], TropicalInt::from(2)),
                ),
                TropicalAutomorphism::new([
                    TropicalRational::polynomial(TropicalPolynomial::monomial(
                        [1, 5],
                        TropicalInt::from(3),
                    )),
                    TropicalRational::polynomial(TropicalPolynomial::monomial(
                        [2, 7],
                        TropicalInt::from(6),
                    )),
                ]),
            ],
            [
                /*
                    a = (2xy³, y)
                    b = (1xy², 2x²y)
                    a o b = (
                        2(1xy²)(2x²y)³ = 9x^7y^5,
                        2x²y
                    )
                */
                TropicalAutomorphism::elementary_triangular(
                    0,
                    TropicalPolynomial::monomial([0, 3], TropicalInt::from(2)),
                ),
                TropicalAutomorphism::monomial(
                    [[1, 2], [2, 1]],
                    [TropicalInt::from(1), TropicalInt::from(2)],
                ),
                TropicalAutomorphism::new([
                    TropicalRational::polynomial(TropicalPolynomial::monomial(
                        [7, 5],
                        TropicalInt::from(9),
                    )),
                    TropicalRational::polynomial(TropicalPolynomial::monomial(
                        [2, 1],
                        TropicalInt::from(2),
                    )),
                ]),
            ],
        ];

        for [alfa, beta, gamma] in test_table {
            assert_eq!(alfa.compose(beta), gamma);
        }
    }

    #[test]
    fn test_compose_3() {
        let test_table: Vec<[TropicalAutomorphism<3>; 3]> = vec![
            [
                /*
                    a(x, y, z) = (5xy³z² + 4xyz, y, z)
                    b(x, y, z) = (x, 3yz², z),
                    (a o b) = (
                        5x(3yz²)³z² + 4x(3yz²)z = 14xy³z^8 + 7xyz³,
                        3yz²,
                        z
                    )
                */
                TropicalAutomorphism::elementary_triangular(
                    0,
                    TropicalPolynomial::from(vec![
                        ([0, 1, 1], TropicalInt::from(4)),
                        ([0, 3, 2], TropicalInt::from(5)),
                    ]),
                ),
                TropicalAutomorphism::elementary_triangular(
                    1,
                    TropicalPolynomial::monomial([0, 0, 2], TropicalInt::from(3)),
                ),
                TropicalAutomorphism::new([
                    TropicalRational::polynomial(TropicalPolynomial::from(vec![
                        ([1, 1, 3], TropicalInt::from(7)),
                        ([1, 3, 8], TropicalInt::from(14)),
                    ])),
                    TropicalRational::polynomial(TropicalPolynomial::monomial(
                        [0, 1, 2],
                        TropicalInt::from(3),
                    )),
                    TropicalRational::polynomial(TropicalPolynomial::variable(2)),
                ]),
            ],
            [
                // https://chatgpt.com/share/66f64893-6760-8008-8833-401786e6c9bd
                TropicalAutomorphism::new([
                    TropicalRational::polynomial(TropicalPolynomial::from(vec![
                        ([1, 1, 3], TropicalInt::from(7)),
                        ([1, 3, 8], TropicalInt::from(14)),
                    ])),
                    TropicalRational::polynomial(TropicalPolynomial::monomial(
                        [0, 1, 2],
                        TropicalInt::from(3),
                    )),
                    TropicalRational::polynomial(TropicalPolynomial::variable(2)),
                ]),
                TropicalAutomorphism::monomial(
                    [[1, 2, 4], [1, 3, 9], [1, 5, 25]],
                    [
                        TropicalInt::from(1),
                        TropicalInt::from(2),
                        TropicalInt::from(3),
                    ],
                ),
                TropicalAutomorphism::new([
                    TropicalRational::polynomial(TropicalPolynomial::from(vec![
                        ([12, 51, 231], TropicalInt::from(45)),
                        ([5, 20, 88], TropicalInt::from(19)),
                    ])),
                    TropicalRational::polynomial(TropicalPolynomial::monomial(
                        [3, 13, 59],
                        TropicalInt::from(11),
                    )),
                    TropicalRational::polynomial(TropicalPolynomial::monomial(
                        [1, 5, 25],
                        TropicalInt::from(3),
                    )),
                ]),
            ],
        ];

        for [alfa, beta, gamma] in test_table {
            assert_eq!(alfa.compose(beta), gamma);
        }
    }

    #[test]
    fn test_compose_triangulars() {
        let triangular: TropicalAutomorphism<3> = TropicalAutomorphism::elementary_triangular(
            0,
            TropicalPolynomial::from(vec![
                ([0, 1, 1], TropicalInt::from(4)),
                ([0, 3, 2], TropicalInt::from(5)),
            ]),
        )
        .compose(TropicalAutomorphism::elementary_triangular(
            1,
            TropicalPolynomial::monomial([0, 0, 2], TropicalInt::from(3)),
        ))
        .compose(TropicalAutomorphism::elementary_triangular(
            2,
            TropicalPolynomial::constant(TropicalInt::from(5)),
        ));

        for (variable, row) in triangular.mappings.iter().enumerate() {
            assert!(row.numerator.terms.iter().all(|(multi_degree, _)| {
                multi_degree.iter().take(variable).all(|d| *d == 0) && multi_degree[variable] == 1
            }))
        }
    }

    #[test]
    fn test_generate_key() {
        let triangular_1: TropicalAutomorphism<3> = TropicalAutomorphism::elementary_triangular(
            0,
            TropicalPolynomial::from(vec![
                ([0, 9, 7], TropicalInt::from(4)),
                ([0, 2, 5], TropicalInt::from(5)),
                ([0, 10, 6], TropicalInt::from(5)),
            ]),
        )
        .compose(TropicalAutomorphism::elementary_triangular(
            1,
            TropicalPolynomial::from(vec![
                ([0, 0, 3], TropicalInt::from(4)),
                ([0, 0, 4], TropicalInt::from(5)),
                ([0, 0, 6], TropicalInt::from(5)),
            ]),
        ))
        .compose(TropicalAutomorphism::elementary_triangular(
            2,
            TropicalPolynomial::constant(TropicalInt::from(5)),
        ));

        let triangular_2: TropicalAutomorphism<3> = TropicalAutomorphism::elementary_triangular(
            0,
            TropicalPolynomial::from(vec![
                ([0, 6, 4], TropicalInt::from(4)),
                ([0, 8, 7], TropicalInt::from(5)),
                ([0, 3, 1], TropicalInt::from(5)),
            ]),
        )
        .compose(TropicalAutomorphism::elementary_triangular(
            1,
            TropicalPolynomial::from(vec![
                ([0, 0, 3], TropicalInt::from(4)),
                ([0, 0, 2], TropicalInt::from(5)),
                ([0, 0, 5], TropicalInt::from(5)),
            ]),
        ))
        .compose(TropicalAutomorphism::elementary_triangular(
            2,
            TropicalPolynomial::constant(TropicalInt::from(3)),
        ));

        let monomial_1 = TropicalAutomorphism::monomial(
            [[5, 4, 1], [3, 7, 5], [1, 8, 9]],
            [
                TropicalInt::from(1),
                TropicalInt::from(3),
                TropicalInt::from(5),
            ],
        );

        let monomial_2 = TropicalAutomorphism::monomial(
            [[2, 1, 3], [6, 5, 1], [1, 4, 2]],
            [
                TropicalInt::from(-5),
                TropicalInt::from(-9),
                TropicalInt::from(7),
            ],
        );

        let monomial_3 = TropicalAutomorphism::monomial(
            [[3, 3, 3], [4, 2, 5], [4, 8, 3]],
            [
                TropicalInt::from(8),
                TropicalInt::from(-1),
                TropicalInt::from(-3),
            ],
        );

        let public_key = monomial_1
            .compose(triangular_1)
            .compose(monomial_2)
            .compose(triangular_2)
            .compose(monomial_3);

        println!("{public_key}");
    }

    #[test]
    fn test_homomorphic_property() {
        let triangular_1: TropicalAutomorphism<3> = TropicalAutomorphism::elementary_triangular(
            0,
            TropicalPolynomial::from(vec![
                ([0, 9, 7], TropicalInt::from(4)),
                ([0, 2, 5], TropicalInt::from(5)),
            ]),
        )
        .compose(TropicalAutomorphism::elementary_triangular(
            1,
            TropicalPolynomial::from(vec![
                ([0, 0, 3], TropicalInt::from(4)),
                ([0, 0, 4], TropicalInt::from(5)),
            ]),
        ))
        .compose(TropicalAutomorphism::elementary_triangular(
            2,
            TropicalPolynomial::constant(TropicalInt::from(5)),
        ));

        let triangular_2: TropicalAutomorphism<3> = TropicalAutomorphism::elementary_triangular(
            0,
            TropicalPolynomial::from(vec![
                ([0, 6, 4], TropicalInt::from(4)),
                ([0, 3, 1], TropicalInt::from(5)),
            ]),
        )
        .compose(TropicalAutomorphism::elementary_triangular(
            1,
            TropicalPolynomial::from(vec![
                ([0, 0, 3], TropicalInt::from(4)),
                ([0, 0, 2], TropicalInt::from(5)),
            ]),
        ))
        .compose(TropicalAutomorphism::elementary_triangular(
            2,
            TropicalPolynomial::constant(TropicalInt::from(3)),
        ));

        let monomial_1 = TropicalAutomorphism::monomial(
            [[5, 4, 1], [3, 7, 5], [1, 8, 9]],
            [
                TropicalInt::from(1),
                TropicalInt::from(3),
                TropicalInt::from(5),
            ],
        );

        let monomial_2 = TropicalAutomorphism::monomial(
            [[2, 1, 3], [6, 5, 1], [1, 4, 2]],
            [
                TropicalInt::from(-5),
                TropicalInt::from(-9),
                TropicalInt::from(7),
            ],
        );

        let monomial_3 = TropicalAutomorphism::monomial(
            [[3, 3, 3], [4, 2, 5], [4, 8, 3]],
            [
                TropicalInt::from(8),
                TropicalInt::from(-1),
                TropicalInt::from(-3),
            ],
        );

        let public_key = monomial_1
            .compose(triangular_1)
            .compose(monomial_2)
            .compose(triangular_2)
            .compose(monomial_3);

        let auto = public_key;

        let u = TropicalPolynomial::monomial([1, 2, 3], TropicalInt::from(4));
        let v = TropicalPolynomial::monomial([0, 5, 7], TropicalInt::from(2));
        let upv = u.clone() + v.clone();

        let cyphertext_u = auto.apply(&u);
        let cyphertext_v = auto.apply(&v);
        let cyphertext_upv = auto.apply(&upv);

        assert_eq!(cyphertext_u + cyphertext_v, cyphertext_upv);
    }

    // FIXME: inverses will not work until we implement TropicalRational::simplify
    // #[test]
    // fn test_inverse_elementary_triangular() {
    //     let triangular = TropicalAutomorphism::elementary_triangular(
    //         0,
    //         TropicalPolynomial::monomial([0, 2, 3], TropicalInt::from(4)),
    //     );

    //     let inverse = TropicalAutomorphism::inverse_elementary_triangular(
    //         0,
    //         TropicalPolynomial::monomial([0, 2, 3], TropicalInt::from(4)),
    //     );

    //     println!("{triangular}");
    //     println!("{inverse}");

    //     assert_eq!(
    //         triangular.compose(inverse),
    //         TropicalAutomorphism::identity()
    //     );
    // }
}
