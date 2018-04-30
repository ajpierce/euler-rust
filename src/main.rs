#![feature(test)]

extern crate test;
use test::Bencher;

mod utils;
// mod p1;
mod p2;

fn main() {
    // p1::fold_sum(10000);
    p2::solve_takewhile(4000000);
}

#[bench]
fn for_sum(b: &mut Bencher) {
    b.iter(|| {
        p2::solve_for(40000000);
    });
}

#[bench]
fn while_sum(b: &mut Bencher) {
    b.iter(|| {
        p2::solve_while(40000000);
    });
}

#[bench]
fn takewhile_sum(b: &mut Bencher) {
    b.iter(|| {
        p2::solve_takewhile(40000000);
    });
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
