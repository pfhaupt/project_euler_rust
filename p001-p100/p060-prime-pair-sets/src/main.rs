// https://projecteuler.net/problem=60

use std::time::Instant;

fn main() {
    let now = Instant::now();
    let old_max = 3;
    let max = 10_001;
    let primes = calc_primes(vec![2], old_max, max);
    let len = primes.len();
    let mut pair_lookup = vec![vec![false; len]; len];
    // Instead of re-calculating all concatenations and pair-status all the time, I can just save it in a lookup table.
    for i in 0..len {
        for j in (i+1)..len {
            let c1 = get_concatenation(primes[i], primes[j]);
            let c2 = get_concatenation(primes[j], primes[i]);
            if is_prime(c1) && is_prime(c2) {
                pair_lookup[i][j] = true;
                pair_lookup[j][i] = true;
            }
        }
    }
    let mut lowest = u64::MAX;
    for i in 0..len {
        for j in (i+1)..len {
            if pair_lookup[i][j] { // set of two primes
                for k in (j+1)..len {
                    if pair_lookup[j][k] && pair_lookup[i][k] { // set of three primes
                        for l in (k+1)..len {
                            if pair_lookup[i][l] && pair_lookup[j][l] && pair_lookup[k][l] { // set of four primes
                                for m in (l+1)..len {
                                    if pair_lookup[i][m] && pair_lookup[j][m] && pair_lookup[k][m] && pair_lookup[l][m] { // set of five primes
                                        let sum = primes[i] + primes[j] + primes[k] + primes[l] + primes[m];
                                        lowest = lowest.min(sum);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{}", lowest);
    println!("{:?}", now.elapsed());
    return;
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