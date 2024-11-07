#![feature(new_range_api)]
#![feature(test)]
extern crate test;

pub mod tropical_automorphism;
pub mod tropical_int;
pub mod tropical_polynomial;
pub mod tropical_rational;

#[cfg(test)]
mod auction;
#[cfg(test)]
mod benchmarks;
