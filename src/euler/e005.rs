use super::wheel_factorization;

pub fn result() -> String {
    format!("{}", smallest_number_divisible_by_one_to(20))
}

/*
Smallest multiple
Problem 5

2520 is the smallest number that can be divided by each of the numbers
from 1 to 10 without any remainder.

What is the smallest positive number that is evenly divisible by all
of the numbers from 1 to 20?
*/
fn smallest_number_divisible_by_one_to(max_n: u64) -> u64 {
    let w = &wheel_factorization::build_factorization_wheel(4);
    let mut all_factors: Vec<u64> = Vec::new();

    for i in 3..max_n {
        let mut i_factors =
            wheel_factorization::prime_factorization_using_wheel(i, w);
        i_factors.sort();
        for f in &all_factors {
            let idx = i_factors.binary_search(&f);
            match idx {
                Ok(x) => i_factors.remove(x),
                _ => 0,
            };
        }
        for j in i_factors {
            all_factors.push(j);
        }
    }

    all_factors.iter().fold(1, |acc, x| acc * x)
}
#[test]
fn euler_005() {
    assert_eq!(smallest_number_divisible_by_one_to(10), 2520);
    assert_eq!(smallest_number_divisible_by_one_to(20), 232792560);
}
