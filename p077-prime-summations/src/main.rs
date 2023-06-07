// https://projecteuler.net/problem=77

use std::time::Instant;

const GOAL: i64 = 5_000;

fn main() {
    let now = Instant::now();
    let mut old_max = 3;
    let mut max = 11;
    let mut primes = calc_primes(vec![2], old_max, max);
    loop {
        for number in old_max..=max {
            let count = count_ways(number, 0, &primes, 0);
            if count >= GOAL {
                println!("{}", number);
                println!("{:?}", now.elapsed());
                return;
            }
        }
        old_max = max;
        max *= 10;
        primes = calc_primes(primes, old_max, max);
    }
}

fn count_ways(goal: i64, acc: i64, primes: &Vec<i64>, start_index: usize) -> i64 {
    if acc == goal {
        return 1;
    } else if acc > goal {
        return 0;
    }
    let mut counter = 0;
    for index in start_index..primes.len() {
        let current_sum = acc + primes[index];
        if current_sum > goal {
            break;
        } else if current_sum == goal {
            // println!("{}", primes[index]);
            counter += 1;
        } else {
            counter += count_ways(goal, current_sum, primes, index);
        }
    }
    counter
}

fn calc_primes(old_primes: Vec<i64>, mut from: i64, limit: i64) -> Vec<i64> {
    if from % 2 == 0 {
        from += 1;
    }
    let mut primes: Vec<i64> = old_primes;
    for number in (from..limit).step_by(2) {
        let sqrt = f64::sqrt(number as f64) as i64;
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