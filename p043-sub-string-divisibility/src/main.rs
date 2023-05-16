use itertools::Itertools;

fn main() {
    // https://projecteuler.net/problem=43
    let numbers: Vec<u64> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut pandigital_numbers = numbers.iter().permutations(numbers.len());
    let mut sum = 0;
    loop {
        if let Some(number) = pandigital_numbers.next() {
            let number = convert(number);
            if is_special(number) {
                sum += number;
            }
        } else {
            break;
        }
    }
    println!("{}", sum);
}

fn convert(number_as_vec: Vec<&u64>) -> u64 {
    let mut result = 0;
    for digit in number_as_vec {
        result = result * 10 + digit;
    }
    result
}

const DIVISORS: [u64; 7] = [17, 13, 11, 7, 5, 3, 2];
fn is_special(mut number: u64) -> bool {
    for divisor in &DIVISORS {
        let last_digits = number % 1000;
        if last_digits % divisor != 0 {
            return false;
        }
        number /= 10;
    }
    true
}