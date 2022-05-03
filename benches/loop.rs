#![feature(test)]

extern crate test;

use rust_playground::loops;

const NUM_OF_LOOP: usize = 1_000;
const NUM_OF_STEP: usize = 100;

#[bench]
fn simple_for(b: &mut test::Bencher) {
    b.iter(|| loops::simple_for(NUM_OF_LOOP));
}

#[bench]
fn multipul_var_with_zip(b: &mut test::Bencher) {
    b.iter(|| loops::multipul_var_with_zip(NUM_OF_LOOP));
}

#[bench]
fn multipul_var_with_zip_and_custom_step(b: &mut test::Bencher) {
    b.iter(|| loops::multipul_var_with_zip_and_custom_step(NUM_OF_LOOP, NUM_OF_STEP));
}

#[bench]
fn multipul_var_with_nested_for_and_custom_step(b: &mut test::Bencher) {
    b.iter(|| loops::multipul_var_with_nested_for_and_custom_step(NUM_OF_LOOP, NUM_OF_STEP));
}
