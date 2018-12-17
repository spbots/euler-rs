use super::prime_utils;
use std::collections::HashSet;

// return unsorted set of prime factors.
pub fn prime_factors(n: u64) -> HashSet<u64> {
    prime_factors_using_wheel(n, &build_factorization_wheel(4))
}

// return vec of prime factors.
pub fn prime_factorization(n: u64) -> Vec<u64> {
    prime_factorization_using_wheel(n, &build_factorization_wheel(4))
}

#[derive(Debug)]
pub struct FactorizationWheel {
    circumference: u64,
    start_primes: Vec<u64>,     // inner wheel numbers
    spoke_increments: Vec<u64>, // spoke increments from (circumference + 1)
}

pub fn build_factorization_wheel(n: usize) -> FactorizationWheel {
    // n is the number of primes to use to construct the wheel.
    // don't bother with gigantic wheels.
    assert!(n > 1 && n <= 5);

    let first_primes = &[2, 3, 5, 7, 11][0..(n as usize)];
    let circumference = first_primes.iter().fold(1, |x, y| x * y);

    // start at the first spoke, primes[n], and do trial division to build the spokes.
    let primes = prime_utils::primes_below(circumference);
    let increments =
        (primes[n as usize]..circumference).fold(vec![0], |mut incs, x| {
            match first_primes.iter().fold(true, |acc, y| acc && (x % y != 0))
            {
                true => incs.push(x - 1),
                false => (),
            }
            incs
        });

    FactorizationWheel {
        circumference: circumference,
        start_primes: primes,
        spoke_increments: increments,
    }
}

// f will be called for each number in the wheel and should return true
// to indicate that the process should continue
fn iterate_wheel<F>(wheel: &FactorizationWheel, upper_bound: u64, mut f: F)
where
    F: FnMut(u64) -> bool,
{
    for i in &wheel.start_primes {
        if !f(*i) || i >= &upper_bound {
            return;
        }
    }

    let mut basis_spoke = wheel.circumference + 1;
    loop {
        for i in &wheel.spoke_increments {
            let spoke = basis_spoke + i;
            if !f(spoke) || spoke >= upper_bound {
                return;
            }
        }
        basis_spoke += wheel.circumference;
    }
}

pub fn check_prime_with_wheel(n: u64, wheel: &FactorizationWheel) -> bool {
    if n == 2 {
        return true;
    }
    let upper_bound = (n as f64).sqrt().ceil() as u64;

    let mut n_is_prime = false;
    iterate_wheel(wheel, upper_bound, |x| {
        n_is_prime = n % x != 0;
        n_is_prime
    });
    n_is_prime
}

pub fn prime_factors_using_wheel(
    n: u64,
    wheel: &FactorizationWheel,
) -> HashSet<u64> {
    if n == 2 {
        return HashSet::new();
    }

    let upper_bound = (n as f64).sqrt().ceil() as u64;
    let mut factors: HashSet<u64> = HashSet::new();
    iterate_wheel(wheel, upper_bound, |x| {
        let d = n / x;
        let r = n % x;
        if r == 0 {
            factors.insert(x);
            factors.insert(d);
        }
        true
    });

    let union_factors: HashSet<u64> = HashSet::new();
    factors.iter().fold(union_factors, |mut acc, &x| {
        if check_prime_with_wheel(x, wheel) {
            acc.insert(x);
        } else {
            let mut pf = prime_factors_using_wheel(x, wheel);
            for i in pf.drain() {
                acc.insert(i);
            }
        }
        acc
    })
}

pub fn prime_factorization_using_wheel(
    n: u64,
    wheel: &FactorizationWheel,
) -> Vec<u64> {
    let pf = prime_factors_using_wheel(n, wheel);
    if pf.is_empty() {
        return vec![n];
    }

    let mut n = n;
    let mut factors = Vec::new();
    while n != 1 {
        for factor in &pf {
            if n % factor == 0 {
                n /= factor;
                factors.push(*factor)
            }
        }
    }

    factors
}

#[test]
fn test_build_wheel() {
    // 2,3,5 wheel, circumference 30
    let w = build_factorization_wheel(3);
    assert_eq!(w.circumference, 30);
    assert_eq!(w.start_primes, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    assert_eq!(w.spoke_increments, [0, 6, 10, 12, 16, 18, 22, 28]);
}

#[test]
fn test_full_factorization() {
    let sorted = |mut x: Vec<u64>| {
        x.sort();
        x
    };
    assert_eq!(sorted(prime_factorization(24)), [2, 2, 2, 3]);
    assert_eq!(sorted(prime_factorization(60)), [2, 2, 3, 5]);
}

#[test]
fn test() {
    assert_eq!(prime_factors(23), [].iter().cloned().collect());
    assert_eq!(prime_factors(24), [2, 3].iter().cloned().collect());
    assert_eq!(prime_factors(25), [5].iter().cloned().collect());
    assert_eq!(prime_factors(26), [2, 13].iter().cloned().collect());
    assert_eq!(
        prime_factors(996699),
        [3, 11, 30203].iter().cloned().collect()
    );
    assert_eq!(
        prime_factors(600851475143),
        [71, 839, 1471, 6857].iter().cloned().collect()
    );
}
