// https://projecteuler.net/problem=37

use std::collections::HashSet;
use std::time::Instant;


fn main() {
    /* 
    The problem boldly claims there are only 11 primes that are truncatable from both sides. Why's that?

    I've spent 4 hours on this problem, my initial thought was sadly a fluke... So I just tried all primes. Well, "all".

    Why are there only 11 primes? Why is there no truncatable prime after 739397?
    -> [Revisit] For a number to be left-right-truncatable, it needs to be left- and right-truncatable, respectively.
                 You can show that both of those sets are finite. The intersection of two finite sets is also finite.

    Also, bottleneck is generating the primes. The rest is super fast, oh well.
    -> [Revisit] I've noticed a few times that HashSet.contains() is very slow. For things like primality, I can just check that on the fly.
    */
    let now = Instant::now();
    const LIMIT: u64 = 1_000_000;
    let pre_calc_primes: Vec<u64> = calc_primes(LIMIT);
    //println!("Calculated primes!");
    let primes_as_set: HashSet<u64> = pre_calc_primes.iter().cloned().collect();
    //println!("Created prime set!");
    let result: u64 = pre_calc_primes.into_iter().filter(|p|truncatable(*p, &primes_as_set)).sum();
    println!("{}", result);
    println!("{:?}", now.elapsed());
}

fn calc_primes(limit: u64) -> Vec<u64> {
    let mut primes: Vec<u64> = vec![];
    for number in 2..limit {
        let sqrt = f64::sqrt(number as f64) as u64;
        let mut prime = true;
        for factor in 2..=sqrt {
            if number % factor == 0 {
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


fn is_prime(candidate: u64, primes: &HashSet<u64>) -> bool {
    primes.contains(&candidate)
}

fn truncatable(number: u64, primes: &HashSet<u64>) -> bool {
    if number < 10 {
        return false;
    }
    let mut temp_number = number;
    while temp_number != 0 {
        if !is_prime(temp_number, primes) {
            return false;
        }
        temp_number /= 10;
    }
    temp_number = number;
    while temp_number != 0 {
        if !is_prime(temp_number, primes) {
            return false;
        }
        let mut number_as_str = temp_number.to_string();
        let _ = number_as_str.remove(0);
        temp_number = number_as_str.parse().unwrap_or_else(|_|0);
    }
    true
}