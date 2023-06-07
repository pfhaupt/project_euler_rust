// https://projecteuler.net/problem=12

use std::time::Instant;

fn main() {
    let now = Instant::now();
    const GOAL: i32 = 500;
    let mut n = 0;
    loop {
        let mut factors = 0;
        let triangle = n * (n + 1) / 2;
        let tri_sqrt = f64::sqrt(triangle as f64) as i64;
        for i in 1..tri_sqrt {
            if triangle % i == 0 {
                factors += 2;
            }
        }
        if factors >= GOAL {
            println!("{}", triangle);
            println!("{:?}", now.elapsed());
            break;
        }
        n += 1;
    }
}
