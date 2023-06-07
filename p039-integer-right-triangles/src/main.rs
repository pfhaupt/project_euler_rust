// https://projecteuler.net/problem=39

use std::time::Instant;

fn main() {
    let now = Instant::now();
    const MAX: usize = 1000;
    let mut result: Vec<usize> = vec![0; MAX + 1];
    for a in 1..=MAX {
        for b in a..=MAX {
            for c in b..=MAX {
                if a * a + b * b == c * c && a + b + c <= MAX {
                    result[a + b + c] += 1;
                }
            }
        }
    }
    let mut best_number = 0;
    let mut best_solutions = 0;
    for i in 0..=MAX {
        if result[i] > best_solutions {
            best_number = i;
            best_solutions = result[i];
        }
    }
    println!("{}", best_number);
    println!("{:?}", now.elapsed());
}
