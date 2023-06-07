// https://projecteuler.net/problem=2

use std::time::Instant;

fn main() {
    let now = Instant::now();
    let mut sum = 0;
    let mut a = 0;
    let mut b = 1;
    let mut c = a + b;
    while c < 4_000_000 {
        c = a + b;
        b = a;
        a = c;
        if c % 2 == 0 {
            sum += c;
        }
    }

    println!("{}", sum);
    println!("{:?}", now.elapsed());
}
