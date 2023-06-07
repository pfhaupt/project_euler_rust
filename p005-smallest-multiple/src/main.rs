// https://projecteuler.net/problem=5

use std::time::Instant;

fn main() {
    let now = Instant::now();
    let mut num: i64 = 1;
    loop {
        let mut valid = true;
        for n in 1..=20 {
            if num % n != 0 {
                valid = false;
                break;
            }
        }
        if valid {
            break;
        }
        num += 1;
    }
    println!("{}", num);
    println!("{:?}", now.elapsed());
}
