// https://projecteuler.net/problem=87

use std::collections::HashSet;

fn main() {
    const GOAL: u64 = 50_000_000;
    let primes = calc_primes(vec![2], 3, f64::sqrt(GOAL as f64) as u64);
    let mut result = HashSet::new();
    for p1 in &primes {
        let square = p1 * p1;
        // Don't need to check if square is greater than GOAL, because p1 only goes up to sqrt(GOAL)
        for p2 in &primes {
            let cube = p2 * p2 * p2 + square;
            if cube >= GOAL {
                break;
            }
            for p3 in &primes {
                let four = p3 * p3 * p3 * p3 + cube;
                if four >= GOAL {
                    break;
                }
                result.insert(four);
            }
        }
    }
    println!("{}", result.len());
}


fn calc_primes(old_primes: Vec<u64>, mut from: u64, limit: u64) -> Vec<u64> {
    if from % 2 == 0 {
        from += 1;
    }
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