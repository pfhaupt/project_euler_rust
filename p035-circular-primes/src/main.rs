// https://projecteuler.net/problem=35

use std::collections::HashSet;

fn main() {
    const MAX: u64 = 1_000_000;
    let mut primes = vec![];
    for number in 2..=MAX {
        if is_prime(number, &primes) {
            primes.push(number);
        }
    }
    let prime_set: HashSet<u64> = primes.iter().cloned().collect();
    let mut result = 0;
    for prime in &primes {
        let rotations = get_rotations(*prime);
        let mut circular = true;
        for candidate in rotations {
            if !prime_set.contains(&candidate) {
                circular = false;
                break;
            }
        }
        if circular {
            result += 1;
        }
    }
    println!("{}", result);
}

fn is_prime(number: u64, primes: &Vec<u64>) -> bool {
    let sqrt = f64::sqrt(number as f64) as u64;
    for prime in primes {
        if prime > &sqrt {
            break;
        }
        if number % prime == 0 {
            return false
        }
    }
    true
}

fn get_rotations(number: u64) -> Vec<u64> {
    let mut result = vec![number];
    let mut temp = number;
    let digit_count = number.to_string().len() as u32 - 1;
    let pow10 = 10u64.pow(digit_count);
    for _ in 0..digit_count {
        let carry = temp % 10;
        temp = temp / 10;
        temp += carry * pow10;
        result.push(temp);
    }
    result
}