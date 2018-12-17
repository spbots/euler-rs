pub fn result() -> String {
    format!("{}", coin_sums(200))
}

/*
Coin sums
Problem 31

In England the currency is made up of pound, £, and pence, p, and there are
eight coins in general circulation:

    1p, 2p, 5p, 10p, 20p, 50p, £1 (100p) and £2 (200p).

It is possible to make £2 in the following way:

    1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p

How many different ways can £2 be made using any number of coins?
*/
fn coin_sums_impl(mut value: u64, coins: &[u64]) -> u64 {
    if coins.len() == 0 {
        return 0; // empty list
    }
    if value == 0 || coins.len() == 1 {
        return 1; // exact match (e.g. 5 - 5) or only pennies available
    }

    // Get all the ways of making "value" out of smaller valued coins
    // e.g. making 7 out of only [2,1] and [1]
    let mut n_sums = coin_sums_impl(value, &coins[1..]);

    while value >= coins[0] {
        value -= coins[0];
        // subtract the largest coin, then see how many ways there are to get
        // to "value" with all the smaller valued coins.
        n_sums += coin_sums_impl(value, &coins[1..]);
    }

    n_sums
}
fn coin_sums(value: u64) -> u64 {
    coin_sums_impl(value, &[200, 100, 50, 20, 10, 5, 2, 1])
}

#[test]
fn euler_031() {
    assert_eq!(coin_sums_impl(2, &[2, 1]), 2);
    assert_eq!(coin_sums_impl(3, &[2, 1]), 2);
    assert_eq!(coin_sums_impl(4, &[2, 1]), 3);
    assert_eq!(coin_sums_impl(7, &[5, 2, 1]), 6);
    assert_eq!(coin_sums_impl(10, &[5, 2, 1]), 10);
    assert_eq!(coin_sums_impl(10, &[10, 5, 2, 1]), 11);
    assert_eq!(coin_sums_impl(15, &[10, 5, 2, 1]), 22);
    assert_eq!(coin_sums_impl(20, &[20, 10, 5, 2, 1]), 41);
    assert_eq!(coin_sums(20), 41);
}
