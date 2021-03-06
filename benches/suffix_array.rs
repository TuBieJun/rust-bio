#![feature(test)]

extern crate bio;
extern crate bit_vec;
extern crate test;

use test::Bencher;

use bio::data_structures::suffix_array::*;

#[bench]
fn bench_suffix_array(b: &mut Bencher) {
    b.iter(|| suffix_array(b"GCCTTAACATTATTACGCCTA$"));
}
