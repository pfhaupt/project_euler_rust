// https://projecteuler.net/problem=7

use std::time::Instant;

fn main() {
    let now = Instant::now();
    let mut prime_counter = 0;
    let mut n = 2;
    loop {
        if is_prime(n) {
            prime_counter += 1;
        }
        if prime_counter == 10001 {
            println!("{}", n);
            println!("{:?}", now.elapsed());
            return;
        }
        n += 1;
    }
}

fn is_prime(n: i32) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}
