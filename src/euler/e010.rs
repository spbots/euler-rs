use super::prime_utils;

pub fn result() -> String {
    format!("{}", sum_of_primes_less_than(2000000))
}

/*
Summation of primes
Problem 10

The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

Find the sum of all the primes below two million.
A: 142913828922
*/
fn sum_of_primes_less_than(n: u64) -> u64 {
    let mut x: u64 = 0;
    prime_utils::for_each_prime(n, |y| x += y);
    x
}
#[test]
fn euler_010() {
    assert_eq!(sum_of_primes_less_than(10), 17);
    assert_eq!(sum_of_primes_less_than(2000000), 142913828922);
}
