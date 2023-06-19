// https://projecteuler.net/problem=41

use std::time::Instant;
use itertools::Itertools;

fn main() {
    let now = Instant::now();
    let mut biggest = 0;
    // Largest pandigital prime is 7 digits long, because all 8- and 9-digit pandigitals are divisible by 3
    let digits = (1..=7).collect_vec();
    let mut perms = digits.iter().permutations(digits.len());
    while let Some(candidate) = perms.next() {
        let mut number = 0;
        for d in candidate {
            number = number * 10 + d;
        }
        if is_prime(number) {
            biggest = biggest.max(number);
        }
    }
    println!("{}", biggest);
    println!("{:?}", now.elapsed());
}

fn is_prime(n: u64) -> bool {
    if n == 2 || n == 3 { return true; }
    if n < 2 || n % 2 == 0 { return false; }
    if n < 9 { return true; }
    if n % 3 == 0 { return false; }
    let r = f64::sqrt(n as f64) as u64;
    let mut f = 5;
    while f <= r {
        if n % f == 0 { return false; }
        if n % (f + 2) == 0 { return false; } 
        f += 6;
    }
    return true;
}
