use super::bit_vector::BitVector;

pub fn primes_below(n: u64) -> Vec<u64> {
    /*
    from https://stackoverflow.com/questions/1042717/
    is-there-a-way-to-find-the-approximate-value-of-the-nth-prime/1069023#1069023
    Wikipedia gives the following upper bound for n >= 6
        p_n <= n log n + n log log n   (1)

    where p_n is the nth prime, and log is the natural logarithm. This is a good
    start, but it can overestimate by a not inconsiderable amount. This article
    in The College Mathematics Journal gives a tighter upper bound for n >= 7022
        p_n <= n log n + n (log log n - 0.9385)   (2)
    */
    let expected_sieve_size = {
        let f = n as f64;
        if n >= 7022 {
            f * f.ln() + f * (f.ln().ln() - 0.9385)
        } else if n >= 6 {
            f * f.ln() + f * (f.ln().ln())
        } else {
            f
        }
    } as usize;

    let mut a: Vec<u64> = Vec::with_capacity(expected_sieve_size);
    for_each_prime(n, |x| a.push(x));
    a
}

pub fn for_each_prime<F>(max_prime: u64, mut f: F)
where
    F: FnMut(u64),
{
    // only keep track of the odd values in the bit vector.
    // if we let sieve[0] = 1, sieve[1] = 3, etc. then we
    // can remove numbers from the sieve by jumping 2*i+1

    let sieve_size = (max_prime / 2) as usize;
    let mut sieve = BitVector::make_vector(sieve_size);
    let index_to_number = |x| 2 * x + 1;

    for i in 1..sieve_size {
        if !sieve.is_bit_set(i) {
            let step_size = index_to_number(i);
            for j in (i + step_size..sieve_size).step_by(step_size) {
                sieve.set_bit(j);
            }
        }
    }

    f(2);
    for i in 1..sieve_size {
        if !sieve.is_bit_set(i) {
            f(index_to_number(i) as u64);
        }
    }
}

#[test]
fn primes_below_100() {
    assert_eq!(
        primes_below(100),
        [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59,
            61, 67, 71, 73, 79, 83, 89, 97,
        ]
    );
}
