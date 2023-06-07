// https://projecteuler.net/problem=47

use std::time::Instant;

fn main() {
    let now = Instant::now();
    const STREAK: u64 = 4;
    let mut max = 11;
    let mut primes = calc_primes(vec![2], 3, 11);
    let mut current = 1;
    loop {
        if 2 * current > max {
            let new_limit = 2 * max + 1;
            primes = calc_primes(primes, max, new_limit);
            max = new_limit;
        }
        let mut valid = true;
        for i in 0..STREAK {
            let n = current + i;
            let prime_factors = get_unique_prime_factors(n, &primes);
            if prime_factors.len() as u64 != STREAK {
                valid = false;
                break;
            }
        }
        if valid {
            break;
        }
        current += 1;
    }
    println!("{}", current);
    println!("{:?}", now.elapsed());
}

fn calc_primes(old_primes: Vec<u64>, from: u64, limit: u64) -> Vec<u64> {
    let mut primes: Vec<u64> = old_primes;
    for number in (from..limit).step_by(2) {
        let sqrt = f64::sqrt(number as f64) as u64;
        let mut prime = true;
        for p in &primes {
            if p > &sqrt {
                break;
            } else if number % p == 0 {
                prime = false;
                break;
            }
        }
        if prime {
            primes.push(number);
        }
    }
    primes
}

fn get_unique_prime_factors(number: u64, primes: &Vec<u64>) -> Vec<u64> {
    let mut result = vec![];
    for p in primes {
        if p > &number {
            break;
        }
        if number % p == 0 {
            result.push(*p);
        }
    }
    result
}