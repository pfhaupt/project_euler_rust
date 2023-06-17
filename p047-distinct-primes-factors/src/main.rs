// https://projecteuler.net/problem=47

use std::time::Instant;

fn main() {
    let now = Instant::now();
    const LIMIT: usize = 1_000_000; // Just a wild guess
    const TARGET_FACTOR_COUNT: i64 = 4;
    const STREAK: usize = 4;
    let mut factor_count = vec![0; LIMIT];
    let mut consecutive = 0;
    for n in 2..LIMIT {
        if factor_count[n] == 0 {
            // found a prime, can't be part of the sequence
            consecutive = 0;
            for multiple in ((2*n)..LIMIT).step_by(n) {
                factor_count[multiple] += 1;
            }
        } else if factor_count[n] == TARGET_FACTOR_COUNT {
            consecutive += 1;
            if consecutive == STREAK {
                println!("{}", n - STREAK + 1);
                println!("{:?}", now.elapsed());
                return;
            }
        } else {
            consecutive = 0;
        }
    }
}
