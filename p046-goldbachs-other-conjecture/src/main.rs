use std::collections::HashSet;

fn main() {
    // https://projecteuler.net/problem=46
    let mut max = 11;
    let mut primes = calc_primes(vec![2], 3, max);
    let mut primes_as_set: HashSet<u64> = primes.iter().cloned().collect();
    let mut n = 9;
    loop {
        if 2 * n > max {
            let new_limit = 2 * max + 1;
            primes = calc_primes(primes, max, new_limit);
            max = new_limit;
            primes_as_set = primes.iter().cloned().collect();
        }
        let mut valid = true;
        for p in &primes {
            if p > &n {
                // Tried all primes up to n, can't be represented as a sum
                valid = false;
                break;
            }
            let diff = n - p;
            let remainder = diff / 2;
            let sqrt = f64::sqrt(remainder as f64) as u64;
            if sqrt * sqrt == remainder {
                break;
            }
        }
        if !valid {
            break;
        }
        n = get_next_composite(n, &primes_as_set);
    }
    println!("{}", n);
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

fn get_next_composite(mut current: u64, primes: &HashSet<u64>) -> u64 {
    current += 2;
    while primes.contains(&current) {
        current += 2;
    }
    current
}
