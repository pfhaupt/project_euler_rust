use std::collections::HashSet;

fn main() {
    // https://projecteuler.net/problem=50
    const MAX: u64 = 1_000_000;
    let primes = calc_primes(vec![2], 3, MAX);
    let primes_as_set: HashSet<u64> = primes.iter().cloned().collect();
    let mut start = 0;
    let mut record_length = 0;
    for index in 0..(primes.len()-record_length) {
        let mut sum = primes[index];
        let mut length = 0;
        for next_index in (index+1)..primes.len() {
            if sum > MAX {
                break;
            }
            if primes_as_set.contains(&sum) {
                length = next_index - index;
            }
            sum += primes[next_index];
        }
        if length > record_length {
            record_length = length;
            start = index;
        }
    }
    let mut result = 0;
    for i in 0..record_length {
        result += primes[start + i];
    }
    println!("{} {} {}", start, record_length, result);
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