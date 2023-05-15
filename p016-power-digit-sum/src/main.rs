use num_bigint::BigUint;

fn main() {
    let n = BigUint::from(2u32);
    let mut result = n.clone();
    for _ in 1..1000 {
        result *= &n;
    }
    let mut sum: BigUint = BigUint::from(0u32);
    let zero: BigUint = BigUint::from(0u32);
    let ten: BigUint = BigUint::from(10u32);
    while result != zero {
        let digit = &result % &ten;
        sum += &digit;
        result = result / &ten;
    }
    println!("{}", sum);
}
