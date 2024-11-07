/*
    README: this is a failed idea on how to perform auctions
    by definining basis points and interpolating polynomials
    it didn't work since applying to a point doesn't commute
    with the automorphism (duh...) read comments for context
*/
use crate::{
    benchmarks::{make_random_2terms_triangular, make_random_monomial},
    tropical_automorphism::TropicalAutomorphism,
    tropical_int::TropicalInt,
    tropical_polynomial::TropicalPolynomial,
};

/*
    I wanted to create this thing where
    one party has no combinatorial bids, so
    p(x + y) = p(x) + p(y) but the other party
    has one so p(x + y) > p(x) + p(y)
*/
fn alice_bob_combinatorial_bids() -> [TropicalPolynomial<2>; 2] {
    const N: usize = 2;
    /*
        a(1, 0) = 5
        a(0, 1) = 5
        a(1, 1) = 11

        a(x, y) = 0x^5 + 0y^5 + (-3)x^7y^7

        a(1, 0) = 5 + 0 + 4 = 5
        a(0, 1) = 0 + 5 + 4 = 5
        a(1, 1) = 5 + 5 + 11 = 11
    */
    let alice_poly: TropicalPolynomial<N> = TropicalPolynomial::from(vec![
        ([5, 0], TropicalInt::Integer(0)),
        ([0, 5], TropicalInt::Integer(0)),
        ([7, 7], TropicalInt::Integer(-3)),
    ]);

    /*
        b(1, 0) = 6
        b(0, 1) = 4
        b(x, y) = 0x^6 + 0y^4
    */
    let bob_poly: TropicalPolynomial<N> = TropicalPolynomial::from(vec![
        ([6, 0], TropicalInt::Integer(0)),
        ([0, 4], TropicalInt::Integer(0)),
    ]);

    [alice_poly, bob_poly]
}

enum ToyAuctionResult {
    #[allow(dead_code)]
    SeparateBidsXY(usize, usize),
    JointBidXY(usize),
    // shouldn't really happen I think...
    AuctionFailed,
}

/*
    the idea was that we could do combinatorial bids
    by evaluating over sums of basis points
    it works in plaintext which is kinda neat
*/
fn evaluate_interpolating_polys<const N: usize>(
    alice_poly: &TropicalPolynomial<N>,
    bob_poly: &TropicalPolynomial<N>,
    winning_poly: &TropicalPolynomial<N>,
    points: [[TropicalInt; N]; 3],
) -> ToyAuctionResult {
    let [point_x, point_y, point_xy] = points;

    let winning_bid_x = winning_poly.evaluate(point_x);
    let winning_bid_y = winning_poly.evaluate(point_y);
    let winning_bid_xy = winning_poly.evaluate(point_xy);

    let xy_won = winning_bid_xy + (winning_bid_x * winning_bid_y);
    if xy_won == winning_bid_xy {
        let alice_bid_xy = bob_poly.evaluate(point_xy);
        let bob_bid_xy = bob_poly.evaluate(point_xy);

        if winning_bid_xy == alice_bid_xy {
            ToyAuctionResult::JointBidXY(0)
        } else if winning_bid_xy == bob_bid_xy {
            ToyAuctionResult::JointBidXY(1)
        } else {
            ToyAuctionResult::AuctionFailed
        }
    } else {
        let alice_bid_x = alice_poly.evaluate(point_x);
        let alice_bid_y = alice_poly.evaluate(point_y);
        let bob_bid_x = bob_poly.evaluate(point_x);
        let bob_bid_y = bob_poly.evaluate(point_y);

        // this is somewhat unfair if it ties but whatever just a test
        match (
            winning_bid_x == alice_bid_x,
            winning_bid_y == alice_bid_y,
            winning_bid_x == bob_bid_x,
            winning_bid_y == bob_bid_y,
        ) {
            (true, true, _, _) => ToyAuctionResult::SeparateBidsXY(0, 0),
            (_, _, true, true) => ToyAuctionResult::SeparateBidsXY(1, 1),
            (true, _, _, true) => ToyAuctionResult::SeparateBidsXY(0, 1),
            (_, true, true, _) => ToyAuctionResult::SeparateBidsXY(1, 0),
            _ => ToyAuctionResult::AuctionFailed,
        }
    }
}

/*
    but the whole idea of evaluating the polynomials
    under the automorphism doesn't work
    alpha(u + v) = alpha(u) + alpha(v)
    but
    alpha(u+v)(s) != alpha((u+v)(s))
*/
#[test]
fn test_homomorphic_combinatorial_auction() {
    const N: usize = 2;
    let [alice_poly, bob_poly] = alice_bob_combinatorial_bids();
    let public_key: TropicalAutomorphism<N> = {
        let a = make_random_monomial().compose(make_random_2terms_triangular());
        let b = make_random_monomial().compose(make_random_2terms_triangular());
        a.compose(b).compose(make_random_monomial())
    };

    let winning_poly = alice_poly.clone() + bob_poly.clone();
    let alice_commit = public_key.apply(&alice_poly);
    let bob_commit = public_key.apply(&bob_poly);
    let winning_commit = alice_commit.clone() + bob_commit.clone();
    assert_eq!(public_key.apply(&winning_poly), winning_commit);

    let point_x = [TropicalInt::Integer(1), TropicalInt::Integer(0)];
    let point_y = [TropicalInt::Integer(0), TropicalInt::Integer(1)];
    let point_xy = [TropicalInt::Integer(1), TropicalInt::Integer(1)];
    let points = [point_x, point_y, point_xy];

    let plaintext_result =
        evaluate_interpolating_polys(&alice_poly, &bob_poly, &winning_poly, points);
    match plaintext_result {
        ToyAuctionResult::JointBidXY(0) => {
            // this is expected
        }
        _ => panic!("unexpected plaintext auction result"),
    }

    assert!(
        (0..5)
            .map(|_| evaluate_interpolating_polys(
                &alice_commit,
                &bob_commit,
                &winning_commit,
                points
            ))
            .any(|ciphertext_result| match ciphertext_result {
                ToyAuctionResult::JointBidXY(0) => false,
                _ => true,
            }),
        "you got lucky and the ciphertext auction got the same result as the plaintext one"
    );
}
