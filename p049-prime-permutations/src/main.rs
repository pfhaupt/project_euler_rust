// https://projecteuler.net/problem=49

use std::collections::HashSet;

fn main() {
    let max = 10001;
    let primes = calc_primes(vec![2], 3, max);
    let primes_as_set: HashSet<u64> = primes.iter().cloned().collect();
    for index in 0..primes.len() {
        let prime = primes[index];
        if prime <= 1487 {
            continue;
        }
        for other_index in (index+1)..primes.len() {
            let other_prime = primes[other_index];
            let diff = other_prime - prime;
            let next_prime = other_prime + diff;
            if primes_as_set.contains(&next_prime) && is_valid_trio(prime, other_prime, next_prime) {
                let result_string = prime.to_string() + &other_prime.to_string() + &next_prime.to_string();
                println!("{}", result_string);
                return;
            }
        }
    }
}

fn is_valid_trio(mut p1: u64, mut p2: u64, mut p3: u64) -> bool {
    let mut matrix: [[u64; 10]; 3] = [[0; 10]; 3];
    while p1 != 0 {
        let digit = p1 % 10;
        matrix[0][digit as usize] += 1;
        p1 /= 10;
    }
    while p2 != 0 {
        let digit = p2 % 10;
        matrix[1][digit as usize] += 1;
        p2 /= 10;
    }
    while p3 != 0 {
        let digit = p3 % 10;
        matrix[2][digit as usize] += 1;
        p3 /= 10;
    }
    for i in 0..10 {
        let e1 = matrix[0][i];
        let e2 = matrix[1][i];
        let e3 = matrix[2][i];
        if e1 != e2 || e2 != e3 || e1 != e3 {
            return false;
        }
    }
    true
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