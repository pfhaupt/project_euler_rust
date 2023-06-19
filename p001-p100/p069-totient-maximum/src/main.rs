// https://projecteuler.net/problem=69

use std::time::Instant;

fn main() {
    let now = Instant::now();
    const GOAL: usize = 1_000_000;
    let phi = calculate_phi(GOAL);
    let mut record_n = 0;
    let mut record_ratio = 0.0;

    for n in 2..=GOAL {
        let ratio = n as f64 / phi[n] as f64;
        if ratio > record_ratio {
            record_n = n;
            record_ratio = ratio;
        }
    }
    println!("{}", record_n);
    println!("{:?}", now.elapsed());
}

fn calculate_phi(limit: usize) -> Vec<usize> {
    // https://cp-algorithms.com/algebra/phi-function.html#etf_1_to_n
    let mut result = vec![0; limit as usize + 1];
    for i in 0..=limit {
        result[i as usize] = i;
    }
    for i in 2..=limit {
        if result[i as usize] == i {
            for j in (i..=limit).step_by(i as usize) {
                result[j as usize] -= result[j as usize] / i;
            }
        }
    }
    result
}