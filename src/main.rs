#![feature(test)]

extern crate num_bigint;
// extern crate test;
// use test::Bencher;

mod utils;
// mod p1;
mod p2;

fn main() {
    // p1::fold_sum(10000);
    p2::solve2(4000000);
}

/*
#[bench]
fn for_sum(b: &mut Bencher) {
    b.iter(|| {
        p1::for_sum(10000);
    });
}

#[bench]
fn fold_sum(b: &mut Bencher) {
    b.iter(|| {
        p1::fold_sum(10000);
    });
}
*/
