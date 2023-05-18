use num_bigint::BigUint;

fn main() {
    // https://projecteuler.net/problem=57
    let mut result = 0;
    let mut numerator = BigUint::from(1u32);
    let mut denominator = BigUint::from(1u32);
    for _ in 0..1000 {
        let new_numerator = 2u32 * &denominator + &numerator;
        let new_denominator = &denominator + &numerator;
        let num_len = new_numerator.to_string().len();
        let den_len = new_denominator.to_string().len();
        if num_len > den_len {
            result += 1;
        }
        numerator = new_numerator;
        denominator = new_denominator;
    }
    println!("{}", result);
}
