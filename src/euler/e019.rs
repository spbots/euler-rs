pub fn result() -> String {
    format!("{}", number_of_sundays(12 * 100)) // 100 years
}

/*
Counting Sundays
Problem 19

You are given the following information, but you may prefer to do some
research for yourself.

    1 Jan 1900 was a Monday.
        - 1 Jan 1901 was a Tuesday!!

    Thirty days has September,
    April, June and November.
    All the rest have thirty-one,
    Saving February alone,
    Which has twenty-eight, rain or shine.
    And on leap years, twenty-nine.

    A leap year occurs on any year evenly divisible by 4, but not
        on a century unless it is divisible by 400.

How many Sundays fell on the first of the month during the twentieth
century (1 Jan 1901 to 31 Dec 2000)?
*/
fn number_of_sundays(n_months: u64) -> u64 {
    // If I wanted to be really lazy, just return n/7 and try a couple
    // values close to there on the website :P

    let mut start_of_month = 2; // sunday = 0, monday = 1...
    let mut year = 1901;
    let mut month = 0;
    let mut n_sundays = 0;

    for _n in 1..n_months {
        start_of_month = (start_of_month + match month {
            0 => 31,
            1 => match year % 4 {
                // don't need to care about century rule
                0 => 29,
                _ => 28,
            },
            2 => 31,
            3 => 30,
            4 => 31,
            5 => 30,
            6 => 31,
            7 => 31,
            8 => 30,
            9 => 31,
            10 => 30,
            11 => 31,
            _ => panic!(""),
        }) % 7;
        month = month + 1;
        if month > 11 {
            month = 0;
            year += 1;
        }
        if start_of_month == 0 {
            n_sundays += 1;
        }
    }
    n_sundays
}
#[test]
fn euler_019() {
    assert_eq!(number_of_sundays(8), 0);
    assert_eq!(number_of_sundays(9), 1); // Sept 1901
    assert_eq!(number_of_sundays(12), 2); // Dec 1901
    assert_eq!(number_of_sundays(17), 2);
    assert_eq!(number_of_sundays(18), 3); // June 1902
    assert_eq!(number_of_sundays(12 * 100), 171);
}
