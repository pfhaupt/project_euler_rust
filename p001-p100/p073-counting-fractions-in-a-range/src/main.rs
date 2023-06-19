// https://projecteuler.net/problem=73

use std::time::Instant;

fn main() {
    let now = Instant::now();
    const LIMIT: u64 = 12_000;
    const N_1: u64 = 1;
    const D_1: u64 = 3;
    const N_2: u64 = 1;
    const D_2: u64 = 2;
    let mut result = count_farey(0, 1, N_1, D_1, LIMIT);    // how many fractions are between N_1/D_1 and 1?
    result -= count_farey(N_1, D_1, N_2, D_2, LIMIT);       // how many fractions are between N_1/D_1 and N_2/D_2?
    result -= 1;                                            // we don't want to count N_2/D_2
    println!("{}", result);
    println!("{:?}", now.elapsed());
}

fn count_farey(mut n_1: u64, mut d_1: u64, mut n_2: u64, mut d_2: u64, limit: u64) -> u64 {
    // Idea: https://en.wikipedia.org/wiki/Farey_sequence#Next_term
    // Generates all farey fractions between n_2/d_2 and 1 and counts how many there are (including 1/1)
    let mut counter = 0;
    while n_2 <= limit {
        let k = (limit + d_1) / d_2;
        (n_1, d_1, n_2, d_2) = (n_2, d_2, k * n_2 - n_1, k * d_2 - d_1);
        counter += 1;
    }
    counter
}