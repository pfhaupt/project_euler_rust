// https://projecteuler.net/problem=89

// https://github.com/FilippoRanza/numerus
use numerus::{int_to_roman_upper, roman_to_int};
use std::fs;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let input =
        fs::read_to_string("src/roman.txt").expect("Something went wrong when loading the file!");
    let lines = input.lines();
    let sum = lines.fold(0, |acc, line| {
        acc + line.len().abs_diff(
            int_to_roman_upper(roman_to_int(line).unwrap())
                .unwrap()
                .len()
        )
    });
    println!("{}", sum);
    println!("{:?}", now.elapsed());
}
