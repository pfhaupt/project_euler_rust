use std::collections::BTreeSet;
use num_bigint::BigUint;

fn main() {
    // https://projecteuler.net/problem=29
    let mut powers: BTreeSet<BigUint> = BTreeSet::new();
    for a in 2..=100 {
        for b in 2..=100 {
            let a = BigUint::from(a as u32);
            let result = a.pow(b);
            powers.insert(result);
        }
    }
    for p in &powers {
        println!("{}", p);
    }

    println!();
    println!("{}", powers.len());
}
