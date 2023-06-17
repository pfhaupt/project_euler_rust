// https://projecteuler.net/problem=74

use std::time::Instant;
use rustc_hash::FxHashSet;

fn main() {
    let now = Instant::now();
    const LIMIT: usize = 1_000_000;
    let lookup_factorials: Vec<usize> = (0..10).map(|n| factorial(n)).collect();
    let mut result = 0;
    for n in 1..LIMIT {
        let mut values = FxHashSet::with_capacity_and_hasher(60, Default::default());
        values.insert(n);
        let mut current = n;
        loop {
            let next = apply_rule(current, &lookup_factorials);
            if values.contains(&next) {
                break;
            }
            values.insert(next);
            current = next;
        }
        if values.len() == 60 {
            result += 1;
        }
    }
    println!("{}", result);
    println!("{:?}", now.elapsed());
}

fn apply_rule(mut n: usize, lookup: &Vec<usize>) -> usize {
    let mut sum = 0;
    while n != 0 {
        sum += lookup[n % 10];
        n /= 10;
    }
    sum
}

fn factorial(n: usize) -> usize {
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}
