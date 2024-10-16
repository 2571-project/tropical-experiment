# benchmarks

run `cargo bench`. it will take a while, but you can comment out the 6d benchmark on `src/benches.rs` to get a feel for the other ones.

```
test benches::bench_3d_monomial_triangular_monomial_composition   ... bench:     126,064.78 ns/iter (+/- 6,843.84)
test benches::bench_3d_triangular_monomial_triangular_composition ... bench:     168,587.50 ns/iter (+/- 7,712.16)
test benches::bench_5d_monomial_triangular_monomial_composition   ... bench:  23,739,208.40 ns/iter (+/- 7,441,439.01)
test benches::bench_5d_triangular_monomial_triangular_composition ... bench:   8,405,787.50 ns/iter (+/- 2,122,415.04)
test benches::bench_6d_triangular_monomial_triangular_composition ... bench: 1,289,363,291.70 ns/iter (+/- 539,216,428.51)
```

also `cargo test -p tropical-experiment test_composition_size` to see the asymptotics.
```
(4d) (triangular o monomial o triangular) [512, 256, 128, 32]
(4d) (monomial o triangular o monomial) [64, 64, 64, 64]
(4d) (monomial o triangular o monomial o triangular o monomial) [4096, 4096, 2048, 4096]
(5d) (triangular o monomial o triangular) [16384, 8192, 4096, 2048, 1024]
(5d) (monomial o triangular o monomial) [1024, 1024, 1024, 1024, 512]
(6d) (triangular o monomial o triangular) [1048576, 524288, 262144, 131072, 65536, 32768]
```
it looks like it's `t = O(n⁴)` where `t` is number of terms and `n` is dimension, but also besides doing `(a (+) b) (^) n = a (^) n (+) b (^) n` we are not simplifying the terms at all, so it could be the case that a lot of these terms are not significant. a very low-hanging technique to reduce terms could be to compare terms multi-degrees and coefficients and check if some terms get dominated by others, it should be possible to do in `O(t)`.
