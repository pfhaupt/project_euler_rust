// https://projecteuler.net/problem=55

use num_bigint::BigUint;
use std::time::Instant;


fn main() {
    let now = Instant::now();
    const GOAL: u64 = 10000;
    let result: usize = (1..=GOAL).into_iter().filter(|n|is_lychrel(*n)).collect::<Vec<u64>>().len();
    println!("{}", result);
    println!("{:?}", now.elapsed());
}

fn is_lychrel(number: u64) -> bool {
    let mut number = BigUint::from(number);
    for _ in 0..50 {
        number += reverse(&number);
        if is_palindrome(&number) {
            return false;
        }
    }
    true
}

fn reverse(number: &BigUint) -> BigUint {
    let mut tmp = number.clone();
    let mut result = BigUint::from(0u32);
    while tmp != BigUint::from(0u32) {
        let digit = &tmp % 10u32;
        result = result * 10u32 + digit;
        tmp /= 10u32;
    }
    result
}

fn is_palindrome(number: &BigUint) -> bool {
    let reverse = reverse(number);
    reverse == *number
}