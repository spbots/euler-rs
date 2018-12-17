pub fn result() -> String {
    format!("{}", paths_n_by_n(20))
}

/*
Lattice paths
Problem 15

Starting in the top left corner of a 2×2 grid, and only being
able to move to the right and down, there are exactly 6 routes
to the bottom right corner.

 +--+--+    r-r-d-d,    d-d-r-r
 |  |  |    r-d-r-d,    d-r-d-r
 +--+--+    r-d-d-r,    d-r-r-d
 |  |  |
 +--+--+

How many such routes are there through a 20×20 grid?
*/
fn paths_n_by_n(n: u64) -> u64 {
    // paths are always 2*n long
    // there are always n rights and n downs
    // this is a permutation (order matters) of 2n choose n:
    //  (2n)! / (n! * (2n - n)!)
    // 40! causes overflow so we unpack the factorial into separate terms
    let mut x = 1;
    for i in 1..(n + 1) {
        x *= n + i;
        x /= i;
    }
    x
}
#[test]
fn euler_015() {
    assert_eq!(paths_n_by_n(2), 6);
    assert_eq!(paths_n_by_n(20), 137846528820);
}
