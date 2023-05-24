// https://projecteuler.net/problem=40

fn main() {
    const MAX: usize = 1000000;
    let mut fraction = String::from("");
    let mut number = 1;
    while fraction.len() < MAX {
        fraction += &number.to_string();
        number += 1;
    }
    let fraction = fraction.as_bytes();
    let mut result = 1;
    let mut index = 1;
    while index <= MAX {
        result *= (fraction[index - 1] as usize) - 48;
        index *= 10;
    }
    println!("{}", result);
}
