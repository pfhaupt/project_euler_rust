// https://projecteuler.net/problem=6

use std::time::Instant;

fn main() {
    let now = Instant::now();
    const N: i32 = 100;
    let mut sum = 0;
    let mut prod = 0;
    for n in 1..=N {
        sum += n * n;
        prod += n;
    }
    prod *= prod;
    println!("{}", prod - sum);
    println!("{:?}", now.elapsed());
}
