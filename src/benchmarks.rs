use rand::Rng as _;
use test::Bencher;

use crate::{
    tropical_automorphism::TropicalAutomorphism,
    tropical_int::TropicalInt,
    tropical_polynomial::{Degree, TropicalPolynomial},
};

pub fn make_random_elementary_triangular_row<const N: usize>(
    i: usize,
) -> ([Degree; N], TropicalInt) {
    let mut rng = rand::thread_rng();
    (
        core::array::from_fn(|j| if j <= i { 0 } else { rng.gen_range(0..=31) }),
        TropicalInt::from(rng.gen_range(-15..15)),
    )
}

pub fn make_random_2terms_triangular<const N: usize>() -> TropicalAutomorphism<N> {
    (0..N - 1).fold(TropicalAutomorphism::identity(), |acc, variable| {
        acc.compose(TropicalAutomorphism::elementary_triangular(
            variable,
            TropicalPolynomial::from(vec![
                make_random_elementary_triangular_row(variable),
                make_random_elementary_triangular_row(variable),
            ]),
        ))
    })
}

// good chance to have det != 0, it's a benchmark anyways
pub fn make_random_monomial<const N: usize>() -> TropicalAutomorphism<N> {
    let mut rng = rand::thread_rng();

    TropicalAutomorphism::monomial(
        core::array::from_fn(|_| core::array::from_fn(|_| rng.gen_range(0..=31))),
        core::array::from_fn(|_| rng.gen_range(-15..15)).map(TropicalInt::from),
    )
}

#[bench]
fn bench_3d_monomial_triangular_monomial_composition(bencher: &mut Bencher) {
    bencher.iter(|| {
        let _ = compose_monomial_triangular_monomial::<3>();
    });
}

#[bench]
fn bench_5d_monomial_triangular_monomial_composition(bencher: &mut Bencher) {
    bencher.iter(|| {
        let _ = compose_monomial_triangular_monomial::<5>();
    });
}

#[bench]
fn bench_3d_triangular_monomial_triangular_composition(bencher: &mut Bencher) {
    bencher.iter(|| {
        let _ = compose_triangular_monomial_triangular::<3>();
    });
}

#[bench]
fn bench_5d_triangular_monomial_triangular_composition(bencher: &mut Bencher) {
    bencher.iter(|| {
        let _ = compose_triangular_monomial_triangular::<5>();
    });
}

#[bench]
fn bench_6d_triangular_monomial_triangular_composition(bencher: &mut Bencher) {
    bencher.iter(|| {
        let _ = compose_triangular_monomial_triangular::<6>();
    });
}

fn compose_triangular_monomial_triangular<const N: usize>() -> TropicalAutomorphism<N> {
    let a = make_random_2terms_triangular();
    let b = make_random_monomial();
    let c = make_random_2terms_triangular();

    let result = a.compose(b).compose(c);
    println!(
        "({N}d) (triangular o monomial o triangular) {:?}",
        result
            .mappings
            .iter()
            .map(|m| m.numerator.terms.len())
            .collect::<Vec<_>>()
    );
    result
}

fn compose_monomial_triangular_monomial<const N: usize>() -> TropicalAutomorphism<N> {
    let a = make_random_monomial();
    let b = make_random_2terms_triangular();
    let c = make_random_monomial();
    let result = a.compose(b).compose(c);
    println!(
        "({N}d) (monomial o triangular o monomial) {:?}",
        result
            .mappings
            .iter()
            .map(|m| m.numerator.terms.len())
            .collect::<Vec<_>>()
    );
    result
}

fn compose_public_key<const N: usize>() -> TropicalAutomorphism<N> {
    let a = make_random_monomial();
    let b = make_random_2terms_triangular();
    let c = make_random_monomial();
    let d = make_random_2terms_triangular();
    let e = make_random_monomial();
    let result = a.compose(b.compose(c)).compose(d.compose(e));
    println!(
        "(4d) (monomial o triangular o monomial o triangular o monomial) {:?}",
        result
            .mappings
            .iter()
            .map(|m| m.numerator.terms.len())
            .collect::<Vec<_>>()
    );
    result
}

#[test]
fn test_composition_size() {
    let _ = compose_triangular_monomial_triangular::<4>();
    let _ = compose_monomial_triangular_monomial::<4>();
    let _ = compose_public_key::<4>();

    let _ = compose_triangular_monomial_triangular::<5>();
    let _ = compose_monomial_triangular_monomial::<5>();

    // README: uncomment if you want to see the asymptotic growth
    // but it takes a while to run... like, a while.
    let _ = compose_triangular_monomial_triangular::<6>();
    // let _ = compose_monomial_triangular_monomial::<6>();
}
