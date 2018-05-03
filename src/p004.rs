#[allow(unused_imports)]
use test::Bencher;

/// Problem 4: Largest palindrome product
///
/// A palindromic number reads the same both ways. The largest palindrome made from the product of
/// two 2-digit numbers is 9009 = 91 × 99.
///
/// Find the largest palindrome made from the product of two 3-digit numbers.

fn is_palindrome(val: String) -> bool {
    let reversed = val.chars().rev().collect::<String>();
    return val == reversed;
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

    println!(
        "The largest palindrome between {} and {} is: {}",
        min, max, answer
    );
}

#[bench]
fn p4(b: &mut Bencher) {
    b.iter(|| {
        solve(900, 1000);
    });
}
