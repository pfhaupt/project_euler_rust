// https://projecteuler.net/problem=20

use num_bigint::BigUint;
use std::time::Instant;


fn main() {
    let now = Instant::now();
    let mut n = BigUint::from(1u32);
    for i in 1..100u32 {
        n = n * i;
    }
    let mut sum = BigUint::from(0u32);
    while n != BigUint::from(0u32) {
        sum += &n % BigUint::from(10u32);
        n = n / BigUint::from(10u32);
    }
    println!("{}", sum);
    println!("{:?}", now.elapsed());
}
