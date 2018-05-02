/// Problem 4: Largest palindrome product
///
/// A palindromic number reads the same both ways. The largest palindrome made from the product of
/// two 2-digit numbers is 9009 = 91 × 99.
///
/// Find the largest palindrome made from the product of two 3-digit numbers.
pub fn naive() {
    println!("The naive solution using nested for loops");
    let mut products = Vec::new();
    for i in 100..500 {
        for j in 500..1000 {
            products.push((i * j).to_string());
        }
    }

    let palindromes: Vec<_> = products
        .iter()
        .filter(|&x| *x == *x.chars().rev().collect::<String>())
        .collect();

    println!("number of products: {}", products.len());
    println!("{:?} palindromes: {:?}", palindromes.len(), palindromes);
}
