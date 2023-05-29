// https://projecteuler.net/problem=25

use num_bigint::BigUint;

fn main() {
    const GOAL: usize = 1000;
    let mut a = BigUint::from(0u32);
    let mut b = BigUint::from(1u32);
    let mut c;
    let mut digit_count = 1;
    let mut index = 0;
    while digit_count < GOAL {
        c = &a + &b;
        b = a;
        a = c.clone();
        digit_count = c.to_string().bytes().len();
        index += 1;
    }
    println!("{}", index);
}
