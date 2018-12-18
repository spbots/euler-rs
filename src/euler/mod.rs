mod bit_vector;
mod prime_utils;
mod wheel_factorization;

mod e001;
mod e002;
mod e003;
mod e004;
mod e005;
mod e006;
mod e007;
mod e008;
mod e009;
mod e010;
mod e011;
mod e012;
mod e013;
mod e014;
mod e015;
mod e017;
mod e019;
mod e024;
mod e031;

pub fn run() {
    for i in 1..100 {
        match solution(i) {
            Some(soln_str) => println!("euler {:03}: {}", i, soln_str),
            None => (),
        }
    }
}

fn solution(n: u64) -> Option<String> {
    match n {
        1 => Some(e001::result()),
        2 => Some(e002::result()),
        3 => Some(e003::result()),
        4 => Some(e004::result()),
        5 => Some(e005::result()),
        6 => Some(e006::result()),
        7 => Some(e007::result()),
        8 => Some(e008::result()),
        9 => Some(e009::result()),
        10 => Some(e010::result()),
        11 => Some(e011::result()),
        12 => Some(e012::result()),
        13 => Some(e013::result()),
        14 => Some(e014::result()),
        15 => Some(e015::result()),
        17 => Some(e017::result()),
        19 => Some(e019::result()),
        24 => Some(e024::result()),
        31 => Some(e031::result()),
        _ => None,
    }
}
