use super::wheel_factorization;

pub fn result() -> String {
    format!("{}", largest_prime_factor_of(600851475143))
}

/*
Largest prime factor
Problem 3

The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?
*/
fn largest_prime_factor_of(n: u64) -> u64 {
    let factors = wheel_factorization::prime_factors(n);
    *factors.iter().max().unwrap()
}
#[test]
fn euler_003() {
    assert_eq!(largest_prime_factor_of(69161), 97);
    assert_eq!(largest_prime_factor_of(600851475143), 6857);
}
