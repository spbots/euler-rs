pub fn result() -> String {
    format!("{:?}", pythagorean_triplet_with_sum(1000))
}

/*
Special Pythagorean triplet
Problem 9

A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
a^2 + b^2 = c^2

For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2

There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.
*/
fn pythagorean_triplet_with_sum(n: u64) -> (u64, u64, u64) {
    /*
    a^2 + b^2 = c^2    and    c = 1000 - (a + b)  so
    a^2 + b^2 = (1000 - (a + b))^2

    we can probably deal with O(n^2) in this case since n is
    under 1000.
    */
    let sq = |x| x * x;
    let satisfies_sum = |a, b| sq(a) + sq(b) == sq(n - (a + b));

    for a in 1..n {
        for b in a..n - a {
            if satisfies_sum(a, b) {
                return (a, b, n - (a + b));
            }
        }
    }
    (0, 0, 0)
}
#[test]
fn euler_009() {
    assert_eq!(pythagorean_triplet_with_sum(12), (3, 4, 5));
    assert_eq!(pythagorean_triplet_with_sum(1000), (200, 375, 425));
}
