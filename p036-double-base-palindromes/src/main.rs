// https://projecteuler.net/problem=36

use std::time::Instant;

fn main() {
    let now = Instant::now();
    let palindromes: Vec<u64> = (0..1000000).into_iter().filter(|n| is_palindrome(*n, 2) && is_palindrome(*n, 10)).collect();
    let result: u64 = palindromes.into_iter().sum();
    println!("{}", result);
    println!("{:?}", now.elapsed());
}

fn is_palindrome(number: u64, base: u64) -> bool {
    let mut palindrome = 0;
    let mut tmp_number = number;
    while tmp_number != 0 {
        let digit = tmp_number % base;
        palindrome = palindrome * base + digit;
        tmp_number /= base;
    }
    palindrome == number
}