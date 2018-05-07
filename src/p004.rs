#[allow(unused_imports)]
use test::Bencher;

/// Problem 4: Largest palindrome product
///
/// A palindromic number reads the same both ways. The largest palindrome made from the product of
/// two 2-digit numbers is 9009 = 91 × 99.
///
/// Find the largest palindrome made from the product of two 3-digit numbers.

fn print_answer(min: u64, max: u64, answer: u64) {
    println!(
        "p004: The largest palindrome made from products between {} and {} is: {}",
        min, max, answer
    );
}

fn is_palindrome_collect(val: String) -> bool {
    let reversed = val.chars().rev().collect::<String>();
    return val == reversed;
}

fn is_palindrome(val: String) -> bool {
    for (i1, i2) in val.chars().zip(val.chars().rev()) {
        if i1 != i2 {
            return false;
        }
    }
    return true;
}

fn is_palindrome_cast(num: u64) -> bool {
    let val = num.to_string();
    for (i1, i2) in val.chars().zip(val.chars().rev()) {
        if i1 != i2 {
            return false;
        }
    }
    return true;
}

fn is_palindrome_cast_deref(num: &u64) -> bool {
    let val = num.to_string();
    for (i1, i2) in val.chars().zip(val.chars().rev()) {
        if i1 != i2 {
            return false;
        }
    }
    return true;
}

pub fn solve_collect(min: u64, max: u64) {
    let answer = (min..max)
        .filter_map(|i| {
            (min..max)
                .map(|j| (j * i))
                .filter(|x| is_palindrome_collect(x.to_string()))
                .max()
        })
        .max()
        .unwrap();
    print_answer(min, max, answer);
}

pub fn solve(min: u64, max: u64) {
    let answer = (min..max)
        .filter_map(|i| {
            (min..max)
                .map(|j| (j * i))
                .filter(|x| is_palindrome(x.to_string()))
                .max()
        })
        .max()
        .unwrap();
    print_answer(min, max, answer);
}

pub fn solve_cast(min: u64, max: u64) {
    let answer = (min..max)
        .filter_map(|i| {
            (min..max)
                .map(|j| (j * i))
                .filter(|x| is_palindrome_cast(*x))
                .max()
        })
        .max()
        .unwrap();
    print_answer(min, max, answer);
}

pub fn solve_cast_deref(min: u64, max: u64) {
    let answer = (min..max)
        .filter_map(|i| {
            (min..max)
                .map(|j| (j * i))
                .filter(|x| is_palindrome_cast_deref(x))
                .max()
        })
        .max()
        .unwrap();
    print_answer(min, max, answer);
}

pub fn solve_naive(min: u64, max: u64) {
    let mut answer: u64 = 0;
    for i in (min..max) {
        for j in (min..max) {
            if is_palindrome((i * j).to_string()) && (i * j) > answer {
                answer = (i * j)
            }
        }
    }
    print_answer(min, max, answer);
}

pub fn solve_naive_less_multiplication(min: u64, max: u64) {
    let mut answer: u64 = 0;
    for i in (min..max) {
        for j in (min..max) {
            let product = i * j;
            if is_palindrome(product.to_string()) && product > answer {
                answer = product;
            }
        }
    }
    print_answer(min, max, answer);
}

#[bench]
fn p4_collect(b: &mut Bencher) {
    b.iter(|| {
        solve_collect(900, 1000);
    });
}

#[bench]
fn p4(b: &mut Bencher) {
    b.iter(|| {
        solve(900, 1000);
    });
}

#[bench]
fn p4_cast(b: &mut Bencher) {
    b.iter(|| {
        solve_cast(900, 1000);
    });
}

#[bench]
fn p4_cast_deref(b: &mut Bencher) {
    b.iter(|| {
        solve_cast_deref(900, 1000);
    });
}

#[bench]
fn p4_chars_naive(b: &mut Bencher) {
    b.iter(|| {
        solve_naive(900, 1000);
    });
}

#[bench]
fn p4_chars_naive_less_multiplication(b: &mut Bencher) {
    b.iter(|| {
        solve_naive(900, 1000);
    });
}
