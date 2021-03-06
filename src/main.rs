#![feature(test)]

extern crate num;
extern crate primes;
extern crate test;

mod p001;
mod p002;
mod p003;
mod p004;
mod p005;
mod utils;

fn main() {
    p001::fold_sum(10000);
    p002::solve_takewhile_filter(4000000);
    p003::solve(600851475143);
    p004::solve(900, 1000);
    p005::solve(10, 20);
}
