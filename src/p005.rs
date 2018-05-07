#[allow(unused_imports)]
use num::Integer;
use test::Bencher;

/// Problem 5: Smallest Multiple
/// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
/// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

fn print_answer(min: u64, max: u64, answer: u64) {
    println!(
        "p005: The smallest number evenly divisible by all numbers between {} and {} is: {}",
        min, max, answer
    );
}

#[allow(dead_code)]
fn solve_single_step(min: u64, max: u64) {
    let mut answer = max.clone();
    loop {
        if (min..max).all(|x| answer % x == 0) {
            break;
        } else {
            answer += 1;
        }
    }
    print_answer(min, max, answer);
}

fn solve_big(min: u64, max: u64) {
    let mut answer = max.clone();
    loop {
        if (min..max).all(|x| answer % x == 0) {
            break;
        } else if (answer % 20 == 0) {
            answer += 20;
        } else {
            answer += 1;
        }
    }
    print_answer(min, max, answer);
}

fn solve_vbig(min: u64, max: u64) {
    let mut answer = max.clone();
    loop {
        if (min..max).all(|x| answer % x == 0) {
            break;
        } else if answer % 20 == 0 && answer % 19 == 0 && answer % 18 == 0 {
            answer += 380_u64.lcm(&18_u64);
        } else if answer % 20 == 0 && answer % 19 == 0 {
            answer += 380;
        } else if answer % 20 == 0 {
            answer += 20;
        } else {
            answer += 1;
        }
    }
    print_answer(min, max, answer);
}

pub fn solve(min: u64, max: u64) {
    let answer = (min..max).fold(1_u64, |acc, x| acc.lcm(&x));
    print_answer(min, max, answer)
}

/*  Takes far too long to complete
#[bench]
fn p5_single(b: &mut Bencher) {
    b.iter(|| {
        solve_single_step(10, 20);
    });
}
*/

#[bench]
fn p5_fold(b: &mut Bencher) {
    b.iter(|| {
        solve(10, 20);
    });
}

#[bench]
fn p5_big_step(b: &mut Bencher) {
    b.iter(|| {
        solve_big(10, 20);
    });
}

#[bench]
fn p5_v_big_step(b: &mut Bencher) {
    b.iter(|| {
        solve_vbig(10, 20);
    });
}
