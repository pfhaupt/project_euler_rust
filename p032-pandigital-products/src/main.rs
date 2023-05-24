// https://projecteuler.net/problem=32

use std::collections::HashSet;

fn main() {
    /* 
    Products can only be 4-digit numbers
    If it were a 3-digit number, both multiplicand and multiplier would have to be 3-digit too to be pandigital
    If it were a 5-digit number, multiplicand and multiplier could be at most 3/1 or 2/2 digits, which both does not result in a product of 5 digits
     */
    let mut pandigital_numbers: HashSet<usize> = HashSet::new();
    for number in 1000..10000 {
        let factors = factorize(number);
        for multiplicand in factors {
            let multiplier = number / multiplicand;
            if is_pandigital(multiplicand, multiplier, number) {
                pandigital_numbers.insert(number);
            }
        }
    }
    println!("{}", pandigital_numbers.iter().sum::<usize>());
}

fn factorize(number: usize) -> Vec<usize> {
    let mut result: Vec<usize> = vec![];
    let sqrt = f64::sqrt(number as f64) as usize;
    for candidate in 1..=sqrt {
        if number % candidate == 0 {
            result.push(candidate);
        }
    }
    result
}

fn is_pandigital(multiplicand: usize, multiplier: usize, number: usize) -> bool {
    let str1 = multiplicand.to_string();
    let str2 = multiplier.to_string();
    let str3 = number.to_string();
    for digit in 1..=9 {
        let str4 = digit.to_string();
        if !(str1.contains(&str4) || str2.contains(&str4) || str3.contains(&str4)) {
            return false;
        }
    }
    return str1.len() + str2.len() + str3.len() == 9
}