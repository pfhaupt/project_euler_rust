// https://projecteuler.net/problem=38

use std::time::Instant;

fn main() {
    /* 
    Only need to check up to 10000, because we always "add" at least two numbers to the string.
    If number is >=10000, the string has at least 10 digits/chars, and can't ever be pandigital.
     */
    let now = Instant::now();
    const MAX: usize = 10_000;
    let pandigitals = generate_pandigitals(MAX);
    let result = pandigitals.into_iter().max().unwrap();
    println!("{}", result);
    println!("{:?}", now.elapsed());
}

fn generate_pandigitals(max: usize) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    for number in 1..max {
        let mut num_as_str = String::from("");
        for n in 1..10 {
            let product = number * n;
            num_as_str += &product.to_string();
            if num_as_str.len() >= 9 {
                break;
            }
        }
        if is_pandigital(&num_as_str) {
            result.push(num_as_str);
        }        
    }

    result
}

fn is_pandigital(number: &String) -> bool {
    for digit in 1..=9 {
        let s = digit.to_string();
        if !number.contains(&s) {
            return false;
        }
    }
    number.len() == 9
}