// https://projecteuler.net/problem=48

use std::time::Instant;

fn main() {
    let now = Instant::now();
    const DIGIT_COUNT: u32 = 10;
    const POW10: u64 = 10u64.pow(DIGIT_COUNT);
    let mut result = 0;
    for n in 1..=1000 {
        let mut sum: u64 = 1;
        for _ in 0..n {
            sum = (sum * n) % POW10;
        }
        result = (result + sum) % POW10;
    }
    println!("{}", result);
    println!("{:?}", now.elapsed());
}
