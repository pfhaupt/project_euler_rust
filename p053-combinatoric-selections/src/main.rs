use num_bigint::BigUint;

fn main() {
    // https://projecteuler.net/problem=53
    let goal: BigUint = BigUint::from(1_000_000u32);
    let mut counter = 0;
    for n in 1..=100 {
        for r in 0..=n {
            let combinations = calculate(n, r);
            if combinations > goal {
                counter += 1;
            }
        }
    }
    println!("{}", counter);
}

fn calculate(n: u64, r: u64) -> BigUint {
    let n_fac = factorial(n);
    let r_fac = factorial(r);
    let n_r_fac = factorial(n - r);
    n_fac / (r_fac * n_r_fac)
}

fn factorial(n: u64) -> BigUint {
    let mut result = BigUint::from(1u32);
    for i in 1..=n {
        result *= i;
    }
    result
}