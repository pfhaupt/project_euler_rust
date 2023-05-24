// https://projecteuler.net/problem=52

use std::iter::successors;

fn main() {
    let mut number = 1;
    loop {
        let mut valid = true;
        for i in 2..=6 {
            let multiple = number * i;
            if !same_digits(number, multiple) {
                valid = false;
                break;
            }
        }
        if valid {
            break;
        }
        number += 1;
    }
    println!("{}", number);
}

fn same_digits(mut number1: u64, mut number2: u64) -> bool {
    let number_one_len = successors(Some(number1), |&n| (n >= 10).then(|| n / 10)).count();
    let number_two_len = successors(Some(number2), |&n| (n >= 10).then(|| n / 10)).count();
    if number_one_len != number_two_len {
        return false;
    }
    let mut digit_one_arr: [u64; 10] = [0; 10];
    let mut digit_two_arr: [u64; 10] = [0; 10];
    while number1 != 0 {
        let digit_one = number1 % 10;
        let digit_two = number2 % 10;
        digit_one_arr[digit_one as usize] += 1;
        digit_two_arr[digit_two as usize] += 1;
        number1 /= 10;
        number2 /= 10;
    }
    for i in 0..10 {
        if digit_one_arr[i] != digit_two_arr[i] {
            return false;
        }
    }
    true
}