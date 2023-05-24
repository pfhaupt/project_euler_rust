// https://projecteuler.net/problem=70

use std::collections::HashSet;

fn main() {
    const GOAL: u64 = 10_000_000;
    let mut record_n = 0;
    let mut record_ratio = f64::MAX;
    for n in 2..=GOAL {
        let phi_n = phi(n);
        if is_permutation(n, phi_n) {
            let ratio = n as f64 / phi(n) as f64;
            if ratio < record_ratio {
                record_n = n;
                record_ratio = ratio;
            }
        }
    }
    println!("{} <-> {}", record_n, phi(record_n));
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

fn is_permutation(a: u64, b: u64) -> bool {
    // From Problem 62
    let mut d_a = [0; 10];
    let mut t_a = a.clone();
    while t_a != 0 {
        d_a[(t_a % 10) as usize] += 1;
        t_a /= 10;
    }
    let mut d_b = [0; 10];
    let mut t_b = b.clone();
    while t_b != 0 {
        d_b[(t_b % 10) as usize] += 1;
        t_b /= 10;
    }
    for i in 0..10 {
        if d_a[i] != d_b[i] {
            return false;
        }
    }
    true
}