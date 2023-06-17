// https://projecteuler.net/problem=5

use std::time::Instant;
use std::collections::HashSet;

fn main() {
    let now = Instant::now();
    const GOAL: i64 = 20;
    let primes: Vec<i64> = vec![2, 3, 5, 7, 11, 13, 17, 19];
    /*
    The following few lines find all unique prime factors of the numbers 1 to 20 and finds the product of those factors.
    For a number to be divisible by all numbers from 1 to 20, it needs to contain all of those prime factors (at least once).
    That's why we can just start at that factor, and repeatedly add that factor to our current guess until we find an answer.
     */
    let step_size: Vec<i64> = (1..=GOAL).map(|n| {
        let mut l = vec![];
        for &p in &primes {
            if n % p == 0 {
                l.push(p);
            }
        }
        l
    }).flatten().collect::<HashSet<i64>>().into_iter().collect();
    let step_size = step_size.iter().fold(1, |sum, x| sum * x);
    let mut num: i64 = step_size;
    loop {
        let mut valid = true;
        for n in 1..=GOAL {
            if num % n != 0 {
                valid = false;
                break;
            }
        }
        if valid {
            break;
        }
        num += step_size;
    }
    println!("{}", num);
    println!("{:?}", now.elapsed());
}
