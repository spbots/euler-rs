pub fn result() -> String {
    format!("{}", millionth_lexicographic_permutation())
}

/*
Lexicographic permutations
Problem 24

A permutation is an ordered arrangement of objects. For example, 3124 is one
possible permutation of the digits 1, 2, 3 and 4. If all of the permutations
are listed numerically or alphabetically, we call it lexicographic order.
The lexicographic permutations of 0, 1 and 2 are:

012   021   102   120   201   210

What is the millionth lexicographic permutation of the digits
0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?
*/
fn lexicographic_permutation(n: u64) -> u64 {
    let factorial = |x| (0..x).fold(1, |acc, y| acc * (y + 1));
    assert!(n < factorial(10));

    let mut available_digits = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut digits = Vec::<u64>::new();

    // the lexicographic permutation of all ten digits has 10! entries,
    // and each starting digit has 9! entries. increment the starting
    // digit by the factorial of available digits, then remove the
    // digit from the set of available digits and move on to the next.
    // this is O(n^2), but both of the loops terminate before n=10.
    let mut new_n = n;
    while available_digits.len() > 0 {
        let mut digit_idx = 0;
        let idx_factorial = factorial(available_digits.len() as u64 - 1);
        while new_n >= idx_factorial {
            digit_idx += 1;
            new_n -= idx_factorial;
        }
        digits.push(available_digits[digit_idx]);
        available_digits.remove(digit_idx);
    }

    let mut result = 0;
    for i in 0..digits.len() {
        result *= 10;
        result += digits[i];
    }
    result
}
fn millionth_lexicographic_permutation() -> u64 {
    lexicographic_permutation(999999)
}
#[test]
fn euler_024() {
    assert_eq!(lexicographic_permutation(0), 123456789);
    assert_eq!(lexicographic_permutation(1), 123456798);
    assert_eq!(lexicographic_permutation(362880), 1023456789);
    assert_eq!(lexicographic_permutation(999999), 2783915460);
}
