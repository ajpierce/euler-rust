#![feature(test)]

extern crate primes;
extern crate test;

mod p001;
mod p002;
mod p003;
mod p004;
mod utils;

fn main() {
    p001::fold_sum(10000);
    p002::solve_takewhile(4000000);
    p003::solve(600851475143);
    p004::solve(900, 1000);
}
