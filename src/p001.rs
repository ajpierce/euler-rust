#[allow(unused_imports)]
use test::Bencher;

/// Problem 1: Multiples of 3 and 5
///
/// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
/// The sum of these multiples is 23.
///
/// Find the sum of all the multiples of 3 or 5 below 1000.

pub fn for_sum(max: i32) {
    let mut sum = 0;
    for x in 1..max {
        sum += if x % 3 == 0 || x % 5 == 0 { x } else { 0 };
    }
    println!("Sum is: {}", sum)
}

pub fn fold_sum(max: i32) {
    let sum = (1..max).fold(0, |sum, x| {
        if x % 3 == 0 || x % 5 == 0 {
            sum + x
        } else {
            sum
        }
    });
    println!("Sum is: {}", sum);
}

#[bench]
fn bench_for_sum(b: &mut Bencher) {
    b.iter(|| {
        for_sum(10000);
    });
}

#[bench]
fn bench_fold_sum(b: &mut Bencher) {
    b.iter(|| {
        fold_sum(10000);
    });
}
