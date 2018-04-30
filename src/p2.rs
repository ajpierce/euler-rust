use utils::fibonacci;

/// Problem 2: Even Fibonacci numbers
///
/// Each new term in the Fibonacci sequence is generated by adding the
/// previous two terms.  By starting with 1 and 2, the first 10 terms will be:
///
/// 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
///
/// By considering the terms in the Fibonacci sequence whose values do not
/// exceed four million, find the sum of the even-valued terms.
pub fn solve_while(limit: u32) {
    let mut sum = 0;
    let mut iterator = fibonacci::fibonacci();
    let mut current = iterator.next();
    while current.unwrap() < limit {
        sum += if current.unwrap() % 2 == 0 {
            current.unwrap()
        } else {
            0
        };
        current = iterator.next();
    }

    println!(
        "The sum of all even numbers in the Fibonacci sequence until {:?} is: {}",
        limit, sum
    )
}

pub fn solve_for(limit: u32) {
    let mut sum = 0;
    for current in fibonacci::fibonacci() {
        if current > limit {
            break;
        }
        sum += if current % 2 == 0 { current } else { 0 }
    }
    println!(
        "The sum of all even numbers in the Fibonacci sequence until {:?} is: {}",
        limit, sum
    )
}
