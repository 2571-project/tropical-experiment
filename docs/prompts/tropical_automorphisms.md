# tropical algebra definitions

## tropical semiring

We are operating on the tropical (max-sum) integers (with negative infinity) we shall call `T`. We have:
a (+) b := max(a, b)
a (*) b := a + b
a (^) n := a * n

Which I guess you could think about as the arithmetic over a logarithmic scale (the arithmetic of exponents). Notably we have some sort of a Frobenius identity:
(a (+) b) (^) n = a (^) n + b (^) n

because, for example:
```
(a (+) b) (*) (a (+) b)
= a (*) a (+) a (*) b (+) b (*) a (+) b (*) b
= a (*) a (+) a (*) b (+) b (*) b
```
and
max(2a, a + b, 2b) = max(2a, 2b)
since
a + b > 2a -> b > a -> 2b > a + b
so
(a (+) b) (^) 2 = a (^) 2 (+) b (^) 2

I will sometimes use infix notation for tropical multiplication, and superscripts for tropical exponents, but I'll try to never use + without surrounding by parenthesis so there's a context cue on whether we are talking standard operations or tropical ones.

And also a few quirks of the system, such as:
- additive identity is -inf
- multiplicative identity is 0
- exponents still distribute (tropically) over multiplication: (ab) (^) n = a (^) n (*) b (^) n
- x (^) 2 (+) x (+) 5 = x (^) 2 (+) 5, as if x > 5 then 2x > x which means it's never the maximum element of that tropical sum

Especially that last one and Frobenius worry me, because they mean multiple different polynomial expressions could have identical valuations as functions, which is why reducing them to a normal form is so important. I guess that's doubly true in the case of rational functions, where's there is also ill-definedness in the numerator and denominator possibly having common factors.

I am building a software that has to operate with big tropical automorphisms, and it's vital that I can reduce them to a normal form whenever possible. But we still haven't implemented that. We will get to that eventually.

## polynomial systems

let's call a tropical polynomial algebra

tropical algebra has a peculiarity of having sidedness in polynomial systems. A one-sided system is of the form `p(x) = y, p: T^n -> T, x: T, y: T`, meaning they apply a polynomial to an unknown input point to get a known output point, while a two-sided system is of the form `p(x) = q(x), p: T^n -> T, q: T^n -> T, x: T`. In more usual algebras these types of systems would be equivalent simply by subtracting both sides from a two-sided system to reduce it to an one-sided system. However since tropical algebra operates over a semiring, you can't subtract. Solving systems of different sidedness have different computational complexities.

What you can do however is dividing both sides, which is why this distinction isn't present in rational function systems `(p (/) q) (x) = 0`.

## automorphisms

tropical polynomial semiring automorphisms are functions of the form `a(x) = (p_i(x))_i, a: (T^n -> T) -> (T^n -> T), p_i: T^n -> T` where we are considering polynomials as `T^n -> T` functions, but it would maybe be more precise to distinguish between that and the polynomial expressions, as we can always quotient that by the valuation map to get expressions equivalent as functions.

### applying and composing automorphisms

applying an automorphism to a polynomial yields another polynomial, where all the variables in the input polynomial get substituted by their images on the corresponding automorphism entry, for example:
```
a(x, y) = (x² (+) y², xy)
p(x, y) = 1 (+) x (+) y (+) xy

(a o p)(x, y) = 1 (+) (x² + y²) (+) (xy) (+) (x² + y²)(xy)
              = 1 (+) x² (+) y² (+) xy (+) x³ (+) y³ (+) x²y (+) xy²
```
where of course you could simplify that considering the equivalences we've gone through earlier, but I left it like that just to showcase the algorithm.

to compose automorphisms we do a very similar procedure, applying the left automorphism to every entry of the right one, for example:
```
a(x, y) = (x (+) y², y)
b(x, y) = (x²y, xy²)

(a o b)(x, y) = a(x²y, xy²)
              = ((x²y) (+) (xy²)², xy²)
              = (x²y (+) x²y⁴, xy²)
```

### inverting automorphisms

tropical monomial automorphisms are ones where every variable gets mapped to a tropical monomial, which is equivalent to a standard linear combination (with possibly constant terms). they are invertible if the multi-degrees matrix of the monomials is invertible as a standard integer matrix.

```
a(x, y) = (3x²y, 4xy²)

multidegrees matrix inverse
|(2, 1), (1, 2)| = 4 - 1 = 3
((2, 1), (1, 2))^(-1) = 1/3 ((2, -1), (-1, 2))

solving like a standard linear system
((2, 1), (1, 2))(x, y) = (s - 3, z - 4)
(x, y) = 1/3 ((2, -1), (-1, 2)) (s - 3, z - 4)
       = 1/3 (2s - z - 2, 2z - s - 5)
       = ((2s - z - 2) / 3, (2z - s - 5) / 3)

so with standard operations
a^(-1)(s, z) = ((2s - z - 2)/3, (2z - s - 5)/3)

and with tropical ones
a^(-1)(s, z) = ((s² (/) 2z) (^) (1/3), (z² (/) 5s) (^) (1/3))
```

elementary triangular automorphisms are ones where all variables get mapped to themselves, except for one of them which gets mapped to itself times a polynomial on the variables of higher index `p_i(x_i) = x_i (x) q(x_j)_{j > i} ; p_i(x_j) = x_j` and triangular automorphisms are compositions of elementary triangular automorphisms. similar to triangular matrices they are trivial to invert as well, since the inverse of an elementary triangular is `p_i^(-1)(x_i) = x_i (\) q({x_j | j > i}) ; p_i(x_j) = x_j` and then you can compose the elementary inverses to get the inverse of a triangular.

```
a(x, y) = (x (*) (1 (+) y²), y)
b(x, y) = (xy, y)

b^(-1)(x, y) = (x (/) y, y)
a^(-1)(x, y) = (x (/) (1 (+) y²), y)

(a o b)^(-1)(x, y) = (b^(-1) o a^(-1))(x, y)
                   = b^(-1)(x (/) (1 (+) y²), y)
                   = (x (/) (y (+) y³), y)
```

notice how in general the inverses of automorphisms are tropical rational functions (i.e. tropical quotient of polynomials) possibly with fractional degrees (radicals).
