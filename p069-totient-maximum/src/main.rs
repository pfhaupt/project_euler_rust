use std::collections::HashSet;

fn main() {
    // https://projecteuler.net/problem=69
    const GOAL: u64 = 1_000_000;
    let mut record_n = 0;
    let mut record_ratio = 0.0;

    for n in 2..=GOAL {
        let ratio = n as f64 / phi(n) as f64;
        if ratio > record_ratio {
            record_n = n;
            record_ratio = ratio;
        }
    }
    println!("{}", record_n);
}

fn phi(n: u64) -> u64 {
    let primes: Vec<u64> = factor(n).into_iter().filter(|f| is_prime(*f)).collect();
    let mut phi = n as f64;
    for p in primes {
        phi *= 1.0 - 1.0 / p as f64;
    }
    phi as u64
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

fn factor(n: u64) -> Vec<u64> {
    let mut l = vec![];
    let r = f64::sqrt(n as f64) as u64;
    for i in 1..=r {
        if n % i == 0 {
            l.push(i);
            l.push(n / i);
        }
    }
    l = l.into_iter().collect::<HashSet<u64>>().into_iter().collect();
    l.sort();
    l
}