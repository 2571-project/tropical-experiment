# tropical-experiment

I wrote this [introduction to the subject](https://publish.obsidian.md/justdu/2571/waylon/tropical+crypto+intro) which explains more or less my thoughts at the time, though I didn't get into the specifics of the protocols I was envisioning.

well, know we have this little experiment repository we can run experiments on and test these ideas.

## mpc auction

basically we can use tropical automorphisms to homomorphically compute the max function. though we can't decrypt the result of the computation, if we have an automorphism `a(u + v) = a(u) + a(v)` so as long as we can encode the bids as tropical polynomials, it should be possible to implement a private auction with this.

## new zero-knowledge foundation

since we have a tropical analogue of zippel's lemma, it is very likely that a tropical polynomial commitment system could work. there's still a lot to think about though, and I think having the homomorphic encryption would benefit this anyways (we can only do pcs on traditional polynomials because we have encryption homomorphic over traditional sum)


## symbolic math libraries

recently the develop branch for sagemath (which is the main branch of sorts for the project) has gotten a few contributions on tropical multivariate polynomials, but since it's based on a semiring it doesn't yet implement factor, which would be the most interesting thing to us. still, we could colaborate with sagemath with the intention of later porting the algorithms to this rust implementation, as performance is an important requirement of the project.
