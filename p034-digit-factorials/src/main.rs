fn main() {
    // https://projecteuler.net/problem=34
    /* 
    Upper bound:
    number = sum(digits!)
    if all digits == 9, we get:
    number = 9! * digit_count
    (10 ^ digit_count) - 1 = 9! * digit_count
    Ex: a 4 digit number can be at most 10^4 - 1 = 9999
        it's sum can be at most 9! * 4 = 362880
    we solve for digit_count (we ask wolframalpha), and get digit_count = 6.3634..
    -> Upper bound is 7 digits, after that the number outgrows the sum
     */
    const DIGITS: u32 = 7;
    const MAX: u64 = 10u64.pow(DIGITS);
    
    // let mut sum = 0;
    // for number in 3..=MAX {
    //     if number == get_sum(number) {
    //         sum += number;
    //     }
    // }

    // I'm slowly getting the hang of it
    let sum: u64 = (3..=MAX).filter(|n| *n == get_sum(*n)).sum();

    println!("{}", sum);
}

fn get_sum(number: u64) -> u64 {
    let mut result = 0;
    for ch in number.to_string().bytes() {
        let digit = (ch as u64) - 48;
        result += factorial(digit);
    }
    result
}

fn factorial(number: u64) -> u64 {
    if number <= 1 {
        return 1;
    }
    let mut result = 1;
    for n in 1..=number {
        result *= n;
    }
    result
}