pub fn result() -> String {
    format!("{}", max_collatz_under_one_million())
}

/*
Longest Collatz sequence
Problem 14

The following iterative sequence is defined for the set of positive integers:

n → n/2 (n is even)
n → 3n + 1 (n is odd)

Using the rule above and starting with 13, we generate the following sequence:
13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1

It can be seen that this sequence (starting at 13 and finishing at 1) contains
10 terms. Although it has not been proved yet (Collatz Problem), it is thought
that all starting numbers finish at 1.

Which starting number, under one million, produces the longest chain?

NOTE: Once the chain starts the terms are allowed to go above one million.
*/
fn collatz_length(mut x: u64) -> u64 {
    let mut n = 1;
    while x != 1 {
        n += 1;
        x = match x % 2 {
            0 => x / 2,
            1 => 3 * x + 1,
            _ => panic!(""),
        }
    }
    n
}
fn max_collatz_under_one_million() -> u64 {
    let mut max_n = 0;
    let mut longest_seq_start = 0;

    for i in 0..500000 {
        let val = 999999 - i;
        let n = collatz_length(val);
        if n > max_n {
            max_n = n;
            longest_seq_start = val;
        }
    }
    longest_seq_start
}
#[test]
fn euler_014() {
    assert_eq!(collatz_length(13), 10);
    assert_eq!(max_collatz_under_one_million(), 837799);
}
