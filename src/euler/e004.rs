use super::wheel_factorization;

pub fn result() -> String {
    format!("{}", largest_palindromic())
}

/*
Largest palindrome product
Problem 4

A palindromic number reads the same both ways. The largest palindrome made from the
product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.
*/
fn reverse_digits(n: u64) -> u64 {
    let mut n = n;
    let mut rev = 0;
    while n > 0 {
        let dig = n % 10;
        rev = rev * 10 + dig;
        n /= 10;
    }
    rev
}
fn is_palindromic_number(n: u64) -> bool {
    n == reverse_digits(n)
}
fn largest_palindromic() -> u64 {
    /*
    potential approaches:
    1. start with a=999 b=999 and decrease
    2. start with n=999^2=998001 and decrease in palindromes,
        trying to find 3 digit factors.
        - 997799, 996699, .. 990099, 989989, 988889

    2 probably makes more sense.
    */
    let mut palindrome = 997799;
    let w = &wheel_factorization::build_factorization_wheel(4);

    loop {
        let factors =
            wheel_factorization::prime_factors_using_wheel(palindrome, w);
        if *factors.iter().max().unwrap() < 1000 {
            // if the max is less than 1000, we can probably assume that either there
            // is at least one three digit factor, OR the two digit factors can be
            // combined to make three digit numbers
            // println!("{:?}", factors);
            return palindrome;
        }
        palindrome -= 1100;
        if !is_palindromic_number(palindrome) {
            palindrome += 990;
            if !is_palindromic_number(palindrome) {
                palindrome += 99;
            }
        }
    }
}
#[test]
fn euler_004() {
    assert_eq!(largest_palindromic(), 992299);
    assert_eq!(reverse_digits(1234), 4321);
    assert!(is_palindromic_number(123454321));
    assert!(is_palindromic_number(12344321));
    assert!(!is_palindromic_number(1234421));
}
