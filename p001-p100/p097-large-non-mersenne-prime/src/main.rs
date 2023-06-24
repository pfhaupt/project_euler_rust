// https://projecteuler.net/problem=97

use std::time::Instant;

fn main() {
    let now = Instant::now();
    let mut k = 7830457;
    let mut b = 2;
    let mut result = 1;
    while k != 0 {
        if k % 2 == 1 {
            result = (result * b) % 10_000_000_000u128;
        }
        b = (b * b) % 10_000_000_000u128;
        k = k / 2;
    }
    result *= 28433;
    result %= 10_000_000_000u128;
    result += 1;
    println!("{}", result);
    println!("{:?}", now.elapsed());
}
