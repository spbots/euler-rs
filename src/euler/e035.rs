use super::prime_utils;

pub fn result() -> String {
    format!("{}", circular_primes_below(1000000).len())
}

/*
Circular primes
Problem 35

The number, 197, is called a circular prime because all rotations of
the digits: 197, 971, and 719, are themselves prime.

There are thirteen such primes below 100:
    2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.

How many circular primes are there below one million?
*/
fn all_rotations(mut n: u64) -> Vec<u64> {
    // does not do any sorting.
    let n_digits = ((n as f64).log10() + 1.0).floor() as u32;
    let divisor = 10_u64.pow(n_digits);

    let mut result = Vec::<u64>::new();
    for _i in 0..n_digits {
        n *= 10;
        let r = n / divisor;
        if n_digits > 1 && (r % 2 == 0 || r % 5 == 0) {
            // short circuit if something is guaranteed to have a
            // non-prime rotation.
            return Vec::new();
        }
        n -= r * divisor;
        n += r;
        result.push(n);
    }
    result
}
fn circular_primes_below(n: u64) -> Vec<u64> {
    let p = prime_utils::primes_below(n);
    let mut circular = Vec::<u64>::new();

    for i in 0..p.len() {
        let mut rotations = all_rotations(p[i]);
        let is_circular = rotations.iter().all(|x| match p.binary_search(x) {
            Ok(_) => true,
            Err(_) => false,
        });
        if is_circular {
            circular.append(&mut rotations);
        }
    }
    circular.sort_unstable();
    circular.dedup();
    circular
}
#[test]
fn euler_035() {
    assert_eq!(
        circular_primes_below(100),
        [2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, 97]
    );
    assert_eq!(circular_primes_below(1000000).len(), 55);
}
