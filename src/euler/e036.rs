pub fn result() -> String {
    format!(
        "{}",
        binary_palindromes_less_than(1000000)
            .iter()
            .fold(0, |acc, &x| match is_base_10_palindromic_number(x) {
                true => acc + x,
                false => acc,
            })
    )
}

/*
Double-base palindromes
Problem 36

The decimal number, 585 = 1001001001 (binary), is palindromic
in both bases.

Find the sum of all numbers, less than one million, which are
palindromic in base 10 and base 2.

(Please note that the palindromic number, in either base, may
not include leading zeros.)
*/
fn reverse_digits(n: u64) -> u64 {
    let mut n = n;
    let mut rev = 0;
    while n > 0 {
        let dig = n % 10;
        rev = rev * 10 + dig;
        n /= 10;
    }
    rev
}
fn is_base_10_palindromic_number(n: u64) -> bool {
    n == reverse_digits(n)
}

fn reverse_bits(mut n: u64) -> u64 {
    // Adapted from https://www.geeksforgeeks.org/
    // write-an-efficient-c-program-to-reverse-bits-of-a-number/
    let mut shift_size: u64 = 64 - 1;
    let mut r = n;

    n >>= 1;
    while n > 0 {
        r <<= 1;
        r |= n & 1;
        n >>= 1;
        shift_size -= 1;
    }
    r <<= shift_size;
    r
}

/*
1000000 b10 -> F4240 b16 -> 11110100001001000000 b2 (20 bits)
how many binary palindromes are there in n bits?
n=1->p=1     (1)
n=2->p=1+1   (11)
n=3->p=2+2   (111, 101)
n=4->p=4+2   (1111, 1001)
n=5->p=6+4   (11111, 11011, 10101, 10001)
n=6->p=10+4  (111111, 110011, 101101, 100001)
n=7->p=14+8  (1111111, 1110111, 1101011, 1100011,
              1011101, 1010101, 1001001, 1000001)
suspicion: n=8->p=22+8, n=9->p=30+16, n=10->p=46+16
probably easiest to generate all binary palindromes < n and then
check if they are also decimal palindromes.

how to generate the palindromes?
ask for all the palindromes with exactly n bits.
generate all odd numbers with n/2 bits
for each of these numbers N
    create R, which is N with the bit order reversed
    right-shift R so that it is either right against N (even n_bits)
      or with a one-bit gap between and add 1/0 in the gap(odd n_bits)
    add R+N to the return vec.

this creates an unsorted vector, but I don't think that matters.

optimization:
    can combine even&odd bit numbers
*/
fn binary_palindromes_with_bits(n_bits: u8, max_val: u64) -> Vec<u64> {
    match n_bits {
        0 => panic!(""),
        1 => vec![0b1],
        2 => vec![0b11],
        _ => (1..(0b1 << (n_bits / 2))).step_by(2).fold(
            Vec::<u64>::new(),
            |mut acc, x| {
                {
                    let mut push_if_in_range = |y| {
                        if y < max_val {
                            acc.push(y);
                        }
                    };
                    let r = reverse_bits(x) >> (64 - n_bits);
                    push_if_in_range(r + x);
                    if n_bits % 2 != 0 {
                        push_if_in_range(r + x + (0b1 << (n_bits / 2)));
                    }
                }
                acc
            },
        ),
    }
}
fn binary_palindromes_less_than(n: u64) -> Vec<u64> {
    let n_bits = (n as f64).log2().ceil() as u8;
    (1..=n_bits).fold(Vec::new(), |mut acc, x| {
        acc.append(&mut binary_palindromes_with_bits(x, n));
        acc
    })
}
#[test]
fn euler_036() {
    assert_eq!(reverse_bits(0xf009999900000000), 0x000000009999900f);
    assert_eq!(
        binary_palindromes_with_bits(5, 0xfff),
        [0b10001, 0b10101, 0b11011, 0b11111]
    );
    assert_eq!(binary_palindromes_less_than(15), [1, 3, 5, 7, 9]);
    assert_eq!(binary_palindromes_less_than(16), [1, 3, 5, 7, 9, 15]);
    assert_eq!(binary_palindromes_less_than(1000000).len(), 1999);
}
