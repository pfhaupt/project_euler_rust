// https://projecteuler.net/problem=7

use std::time::Instant;

fn main() {
    let now = Instant::now();
    let mut prime_counter = 1;
    let mut n = 3;
    loop {
        if is_prime(n) {
            prime_counter += 1;
        }
        if prime_counter == 10001 {
            println!("{}", n);
            println!("{:?}", now.elapsed());
            return;
        }
        n += 2;
    }
}



fn is_prime(n: u64) -> bool {
    if n == 2 || n == 3 { return true; }
    if n < 2 || n % 2 == 0 { return false; }
    if n < 9 { return true; }
    if n % 3 == 0 { return false; }
    let r = f64::sqrt(n as f64) as u64;
    let mut f = 5;
    while f <= r {
        if n % f == 0 { return false; }
        if n % (f + 2) == 0 { return false; } 
        f += 6;
    }
    return true;
}
