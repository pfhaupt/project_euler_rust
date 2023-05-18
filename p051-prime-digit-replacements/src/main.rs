use std::collections::HashSet;
use std::iter::successors;
use itertools::Itertools;

const ASTERISK: &str = "*";
const DIGIT_AS_STR: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
fn main() {
    // https://projecteuler.net/problem=51
    let mut old_max = 3;
    let mut max = 1_000_001;
    let mut primes = calc_primes(vec![2], old_max, max);
    let mut primes_as_set: HashSet<u64> = primes.iter().cloned().collect();
    const GOAL: usize = 8;
    loop {
        'prime_loop: for prime in &primes {
            let prime_len = successors(Some(*prime), |&n| (n >= 10).then(|| n / 10)).count();
            let combinations = generate_combinations(*prime);
            for combination in &combinations {
                let mut counter = 0;
                let mut family = vec![];
                for digit in 0..=9 {
                    let replacement = combination.replace("*", &DIGIT_AS_STR[digit]);
                    let parsed_number = replacement.parse();
                    if parsed_number.is_ok() {
                        let n: u64 = parsed_number.unwrap();
                        if n > max {
                            // We can't check if n is a prime because we haven't calculated primes that high, break and try again.
                            break 'prime_loop;
                        }
                        let parsed_len = successors(Some(n), |&n| (n >= 10).then(|| n / 10)).count();
                        if prime_len == parsed_len && is_prime(n, &primes_as_set) {
                            counter += 1;
                            family.push(n);
                        }
                    } else {
                        panic!("{} could not be parsed.", replacement);
                    }
                    if counter == GOAL {
                        println!("Found a prime with a family of {}", GOAL);
                        println!("Family: {:#?}", family);
                        println!("{}", family.iter().min().unwrap());
                        return;
                    }
                }
            }
        }
        old_max = max;
        max = 10 * old_max + 1;
        primes = calc_primes(primes, old_max, max);
        primes_as_set = primes.iter().cloned().collect();
    }
}

fn generate_combinations(number: u64) -> Vec<String> {
    let len = successors(Some(number), |&n| (n >= 10).then(|| n / 10)).count();
    let number_as_str = String::from(number.to_string());
    let number_chars: Vec<String> = number_as_str.chars().map(|c| String::from(c)).collect();

    let mut return_vec = vec![];
    for i in 1..len {
        let mut mask = vec![false; len];
        for j in 0..i {
            mask[len-j-1] = true;
        }
        let mut combinations = mask.iter().permutations(len).unique();
        while let Some(comb) = combinations.next() {
            let mut result = String::from("");
            for index in 0..len {
                if *comb[index] {
                    result += &ASTERISK;
                } else {
                    result += &number_chars[index];
                }
            }
            return_vec.push(result);
        }
    }
    return_vec
}

fn is_prime(candidate: u64, primes: &HashSet<u64>) -> bool {
    primes.contains(&candidate)
}

fn calc_primes(old_primes: Vec<u64>, mut from: u64, limit: u64) -> Vec<u64> {
    print!("Calculating primes between {} and {}... ", from, limit);
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
    println!("Done!");
    primes
}