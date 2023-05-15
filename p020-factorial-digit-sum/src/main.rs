use num_bigint::BigUint;

fn main() {
    let mut n = BigUint::from(1u32);
    for i in 1..100u32 {
        n = n * i;
    }
    let mut sum = BigUint::from(0u32);
    while n != BigUint::from(0u32) {
        sum += &n % BigUint::from(10u32);
        n = n / BigUint::from(10u32);
    }
    println!("{}", sum);
}
