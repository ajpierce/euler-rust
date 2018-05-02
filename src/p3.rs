use primes::PrimeSet;

/// Problem 3: Largest prime factor
///
/// The prime factors of 13195 are 5, 7, 13 and 29.
///
/// What is the largest prime factor of the number 600851475143 ?
pub fn solve3(num: u64) {
    let max_check = (num as f64).sqrt() as u64 + 1;
    let gpf = PrimeSet::new()
        .iter()
        .take_while(|x| x <= &max_check)
        .fold(0, |gpf, x| if num % x == 0 { x } else { gpf });

    println!("Greatest prime factor of {} is: {}", num, gpf);
}
