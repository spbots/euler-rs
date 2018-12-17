pub fn result() -> String {
    format!("{}", sum_3_5_below(1000))
}

/*
Multiples of 3 and 5
Problem 1

If we list all the natural numbers below 10 that are multiples of 3 or 5,
we get 3, 5, 6 and 9. The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.
A: 233168
*/
fn sum_3_5_below(n: u64) -> u64 {
    // this could also be further optimized
    let acc = |bound| (1..bound + 1).fold(0, |x, y| x + y);
    let n = n - 1;

    let x_15 = 15 * acc(n / 15);
    let x_5 = 5 * acc(n / 5);
    let x_3 = 3 * acc(n / 3);

    // subtract x_15 before adding to avoid potential overflow.
    x_3 - x_15 + x_5
}
#[test]
fn euler_001() {
    assert_eq!(sum_3_5_below(1000), 233168);
}
