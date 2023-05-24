// https://projecteuler.net/problem=56

use num_bigint::BigUint;

fn main() {
    let mut record = BigUint::from(0u32);
    for a in 1..100u32 {
        for b in 1..100u32 {
            let base = BigUint::from(a);
            let mut pow = base.pow(b);
            let mut digit_sum = BigUint::from(0u32);
            while pow != BigUint::from(0u32) {
                let digit = &pow % 10u32;
                digit_sum += digit;
                pow /= 10u32;
            }
            if digit_sum > record {
                record = digit_sum;
            }
        }
    }
    println!("{}", record);
}
