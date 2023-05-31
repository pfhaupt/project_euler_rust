// https://projecteuler.net/problem=70

fn main() {
    const GOAL: usize = 10_000_000;
    let phi = calculate_phi(GOAL);
    let mut record_n = 0;
    let mut record_ratio = f64::MAX;
    for n in 2..=GOAL {
        let phi_n = phi[n];
        let ratio = n as f64 / phi_n as f64;
        if ratio < record_ratio && is_permutation(n, phi_n){
            record_n = n;
            record_ratio = ratio;
        }
    }
    println!("{}", record_n);
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

fn is_permutation(a: usize, b: usize) -> bool {
    // From Problem 62
    let mut d_a = [0; 10];
    let mut t_a = a.clone();
    while t_a != 0 {
        d_a[t_a % 10] += 1;
        t_a /= 10;
    }
    let mut d_b = [0; 10];
    let mut t_b = b.clone();
    while t_b != 0 {
        d_b[t_b % 10] += 1;
        t_b /= 10;
    }
    for i in 0..10 {
        if d_a[i] != d_b[i] {
            return false;
        }
    }
    true
}