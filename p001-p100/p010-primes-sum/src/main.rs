// https://projecteuler.net/problem=10

use std::time::Instant;

fn main() {
    let now = Instant::now();
    const LIMIT: u64 = 2_000_000;
    let mut sum: u64 = 0;
    for n in 0..=LIMIT {
        if is_prime(n) {
            sum += n;
        }
    }
    println!("{}", sum);
    println!("{:?}", now.elapsed());
}

fn is_prime(n: u64) -> bool {
    if n == 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let sqrt = f64::sqrt(n as f64);
    for i in (3..=(sqrt as u64)).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}