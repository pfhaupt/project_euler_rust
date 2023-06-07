// https://projecteuler.net/problem=60

use std::time::Instant;

fn main() {
    let now = Instant::now();
    let mut old_max = 3;
    let mut max = 10_001;
    let mut primes = calc_primes(vec![2], old_max, max);
    let mut result: Vec<(Vec<u64>, usize)>;
    loop {
        result = generate_prime_pairs(&primes, 5);
        if result.len() > 0 {
            break;
        }
        old_max = max;
        max = 2 * max + 1;
        primes = calc_primes(primes, old_max, max);
    }
    println!("{}", result[0].0.iter().sum::<u64>());
    println!("{:?}", now.elapsed());
}

fn generate_prime_pairs(primes: &Vec<u64>, size: usize) -> Vec<(Vec<u64>, usize)> {
    let len = primes.len();
    let mut result = vec![];
    if size == 1 {
        for i in 0..len {
            let prime = &primes[i];
            result.push((vec![*prime], i));
        }
    } else {
        let smaller_pairs = generate_prime_pairs(primes, size - 1);
        for tuple in smaller_pairs {
            let pair = tuple.0;
            let highest = tuple.1;
            for i in highest..len {
                let prime = &primes[i];
                let valid = check_pair_status(&pair, prime);
                if valid {
                    let mut new_pair = pair.clone();
                    new_pair.push(*prime);
                    result.push((new_pair, i));
                }
            }
        }
    }
    result
}

fn check_pair_status(pair: &Vec<u64>, candidate: &u64) -> bool {
    for prime in pair {
        let left_concat = get_concatenation(*candidate, *prime);
        if !is_prime(left_concat) {
            return false;
        }
        let right_concat = get_concatenation(*prime, *candidate);
        if !is_prime(right_concat) {
            return false;
        }
    }
    true
}

fn get_concatenation(left_side: u64, right_side: u64) -> u64 {
    let mut len = 0;
    let mut tmp = right_side;
    while tmp != 0 {
        len += 1;
        tmp /= 10;
    }
    let mut pow10 = 1;
    for _ in 0..len {
        pow10 *= 10;
    }
    left_side * pow10 + right_side
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