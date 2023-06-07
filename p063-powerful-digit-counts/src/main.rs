// https://projecteuler.net/problem=63

use std::iter::successors;
use std::time::Instant;


fn main() {
    let now = Instant::now();
    let mut n = 1;
    let mut result = 0;
    loop {
        let mut new_powers = 0;
        for digit in 1..10u128 {
            let pow = digit.pow(n);
            let digit_count = successors(Some(pow), |&n| (n >= 10).then(|| n / 10)).count() as u32;
            if digit_count == n {
                new_powers += 1;
            }
        }
        if new_powers == 0 {
            break;
        }
        result += new_powers;
        n += 1;
    }
    println!("{}", result);
    println!("{:?}", now.elapsed());
}