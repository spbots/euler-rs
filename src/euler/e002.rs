pub fn result() -> String {
    format!("{}", e002_even_fib_sum(4000000))
}

/*
Even Fibonacci numbers
Problem 2

Each new term in the Fibonacci sequence is generated by adding the previous
two terms. By starting with 1 and 2, the first 10 terms will be:

1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...

By considering the terms in the Fibonacci sequence whose values do not exceed
four million, find the sum of the even-valued terms.
A: 4613732
*/
fn e002_even_fib_sum(max_n: u64) -> u64 {
    let mut sum = 0;
    let mut even = 2;
    let mut odd_1 = 3;
    let mut odd_2 = 5;

    while even <= max_n {
        sum += even;
        even = odd_1 + odd_2;
        odd_1 = odd_2 + even;
        odd_2 = even + odd_1;
    }
    sum
}
#[test]
fn euler_002() {
    assert_eq!(e002_even_fib_sum(34), (34 + 8 + 2));
    assert_eq!(e002_even_fib_sum(4000000), 4613732);
}