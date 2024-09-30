# tropical-experiment

I wrote this [introduction to the subject](https://publish.obsidian.md/justdu/2571/waylon/tropical+crypto+intro) which explains more or less my thoughts at the time, though I didn't get into the specifics of the protocols I was envisioning.

well, know we have this little experiment repository we can run experiments on and test these ideas.

## mpc auction

basically we can use tropical automorphisms to homomorphically compute the max function. though we can't decrypt the result of the computation, if we have an automorphism `a(u + v) = a(u) + a(v)` so long as we can encode the bids as tropical polynomials, it should be possible to implement a private auction with this. it is quintessential though, that we can guarantee all the encrypted polynomials are in a "normal form" of sorts, because we want to compare the cyphertexts — think of this more as a digest function than actual homomorphic encryption.

## decryption

before we implement decryption — which is not by any means going to be more than a theoretical tool, as it will require exponentially more compute than encryption — we need be able to divide[^3][^4] tropical polynomials, or convert tropical rational functions to a standard form[^2].

## new zero-knowledge foundation

since we have a tropical analogue of zippel's lemma[^5], it is possible that a tropical polynomial commitment system could work. there's still a lot to think about though, and I think having the homomorphic encryption would benefit this anyways (we can only do pcs on traditional polynomials because we have encryption homomorphic over traditional sum)

## hardness

tropical systems have this particular feature of coming in one-sided and two-sided variants. a one-sided tropical polynomial system is one where you have `P(x) = y`, meaning polynomials on one side and variables on the other, while a two-sided one is `P(x) = Q(y)`. these have different complexities. one-sided systems have been shown to be worst-case NP-complete, but in degree two the generic case is polynomial[^1].

also, it's not clear to me how much more difficult are the tropical rational function systems instead of polynomial ones.

inverting the automorphism is probably unfeasible anyways, as the inverse may have an exponentially larger degree. I suspect inverting the automorphism is equivalent to a two-sided system. however, there is an alternative failure mode for this protocol: solving the system for the plaintext when having both the cyphertext and the public key.

that's a one-sided system, and we already know that at least with some parameters that has been shown to be unsafe. in mpc however it would be possible to hide the key, and at that point I am confident the system would be secure. there's also the possibility of making the cyphertext be another automorphism, which would probably make the system harder to solve again, but these are things I have yet to think more thoroughly about.

## symbolic math libraries

recently the develop branch for sagemath (which is the main branch of sorts for the project) has gotten a few contributions on tropical multivariate polynomials, but since it's based on a semiring it doesn't yet implement factor, which would be the most interesting thing to us. still, we could colaborate with sagemath with the intention of later porting the algorithms to this rust implementation, as performance is an important requirement of the project.

there's also this one in julia that I didn't spend too much time looking into: [Tropicalization of polynomial ideals
](https://docs.oscar-system.org/stable/TropicalGeometry/tropicalization). it appears to have more useful theoretical tools, that could help us with implementing the division algorithm, among others.


## a bit of bibliography:

most of the original texts I'm based on can be found [here](https://publish.obsidian.md/justdu/2571/waylon/tropical+crypto+intro). the following are materials I'm studying for the next steps I've described in this document:
[^1]: [On complexity of the problem of solving systems of tropical polynomial equations of degree two](https://eprint.iacr.org/2024/576)
[^2]: [Minimal Representations of Tropical Rational Functions](https://arxiv.org/abs/2205.05647)
[^3]: [Revisiting Tropical Polynomial Division: Theory, Algorithms and Application to Neural Networks](https://arxiv.org/abs/2306.15157)
[^4]: [Linear and Rational Factorization of Tropical Polynomials](https://arxiv.org/abs/1707.03332)
[^5]: [Tropical Combinatorial Nullstellensatz and Sparse Polynomials](https://hal.science/hal-03043503/file/tropical_combinatorial.pdf)
