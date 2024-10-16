#![feature(new_range_api)]
#![feature(test)]
extern crate test;

mod tropical_automorphism;
mod tropical_int;
mod tropical_polynomial;
mod tropical_rational;

#[cfg(test)]
mod benchmarks;
