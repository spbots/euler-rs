pub fn result() -> String {
    format!("{}", sum_square_diff(100))
}

/*
Sum square difference
Problem 6

The sum of the squares of the first ten natural numbers is,
1^2 + 2^2 + ... + 10^2 = 385

The square of the sum of the first ten natural numbers is,
(1 + 2 + ... + 10)^2 = 55^2 = 3025

Hence the difference between the sum of the squares of the first ten natural
numbers and the square of the sum is 3025 − 385 = 2640.

Find the difference between the sum of the squares of the first one hundred
natural numbers and the square of the sum.
A: 25164150
*/
fn sum_square_diff(n: u64) -> u64 {
    let mut sum = 0;
    let mut sq = 0;
    let mut i = 1;
    while i <= n {
        sum += i;
        sq += i * i;
        i += 1;
    }
    (sum * sum) - sq
}
#[test]
fn euler_006() {
    assert_eq!(sum_square_diff(10), 2640);
    assert_eq!(sum_square_diff(100), 25164150);
}
