pub fn result() -> String {
    format!("{}", sum_of_letters())
}

/*
Number letter counts
Problem 17

If the numbers 1 to 5 are written out in words: one, two, three, four, five, then
there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.

If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words,
how many letters would be used?

NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two)
contains 23 letters and 115 (one hundred and fifteen) contains 20 letters. The use
of "and" when writing out numbers is in compliance with British usage.
*/
fn letters_in_digit(n: u64) -> u64 {
    match n {
        0 => 0, // unpronounced
        1 => 3, // one
        2 => 3, // two
        3 => 5, // three
        4 => 4, // four
        5 => 4, // five
        6 => 3, // six
        7 => 5, // seven
        8 => 5, // eight
        9 => 4, // nine
        _ => panic!(""),
    }
}
fn letters_in_two_digit(n: u64) -> u64 {
    let tens = n / 10;
    let last_digit = n % 10;
    match tens {
        0 => letters_in_digit(last_digit),
        1 => match n {
            10 => 3, // ten
            11 => 6, // eleven
            12 => 6, // twelve
            13 => 8, // thirteen
            14 => 8, // fourteen
            15 => 7, // fifteen
            16 => 7, // sixteen
            17 => 9, // seventeen
            18 => 8, // eighteen
            19 => 8, // nineteen
            _ => panic!(""),
        },
        2 => 6 + letters_in_digit(last_digit), // twenty-<n>
        3 => 6 + letters_in_digit(last_digit), // thirty-<n>
        4 => 5 + letters_in_digit(last_digit), // forty-<n>
        5 => 5 + letters_in_digit(last_digit), // fifty-<n>
        6 => 5 + letters_in_digit(last_digit), // sixty-<n>
        7 => 7 + letters_in_digit(last_digit), // seventy-<n>
        8 => 6 + letters_in_digit(last_digit), // eighty-<n>
        9 => 6 + letters_in_digit(last_digit), // ninety-<n>
        _ => panic!(""),
    }
}
fn number_of_letters(n: u64) -> u64 {
    let mut num_letters = 0;
    num_letters += letters_in_digit(n / 100); // hundreds digit
    if n >= 100 {
        num_letters += 7; // "..hundred.."
        if n % 100 != 0 {
            num_letters += 3; // "..hundred AND.."
        }
    }
    num_letters += letters_in_two_digit(n % 100);
    num_letters
}
fn sum_of_letters() -> u64 {
    let sum = (0..1000).fold(0, |acc, x| acc + number_of_letters(x));
    sum + 11 // add "one thousand"
}
#[test]
fn euler_017() {
    assert_eq!(number_of_letters(1), 3); // one
    assert_eq!(number_of_letters(7), 5); // seven
    assert_eq!(number_of_letters(100), 10); // one hundred
    assert_eq!(number_of_letters(101), 16); // one hundred and one
    assert_eq!(number_of_letters(115), 20); // one hundred and fifteen
    assert_eq!(number_of_letters(342), 23); // three hundred and forty two
}
