use super::prime_utils;

pub fn result() -> String {
    format!("{}", nth_prime(10001))
}

/*
10001st prime
Problem 7

By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that
the 6th prime is 13.

What is the 10001st prime number?
A: 104743
*/
fn nth_prime(n: u64) -> u64 {
    // how many numbers do we need to guarantee that we get at least n primes?
    let f = n as f64;
    prime_utils::primes_below((1.5 * f * f.ln()) as u64)[(n - 1) as usize]
}
#[test]
fn euler_007() {
    assert_eq!(nth_prime(10001), 104743);
}
