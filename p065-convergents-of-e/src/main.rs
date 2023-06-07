// https://projecteuler.net/problem=65

use num_bigint::BigUint; // Numbers get bigger than u128
use std::time::Instant;


fn main() {
    /*
    8th term -> generate_e(7)
    Using a normal Fraction struct (now deleted), I generated those values:
    depth   value   cont_frac[depth]
    6       1/1     1     
    5       1/2     1
    4       2/9     4
    3       9/11    1
    2       11/20   1
    1       20/51   2
    0       51/71   1

    result=193/71 (after adding 2 * 71/71 to it)
    
    Idea:
    It's easy to see that every numerator is just the previous denominator.
    Every denominator follows a simple formula:
    d_n = d_(n+1) * cont_frac[n] + n_(n+1)
    example:
    d_4 = d_5 * cont_frac[4] + n_5 = 2 * 4 + 1 = 9
    n_4 = d_5 = 2
    d_3 = d_4 * cont_frac[3] + n_4 = 9 * 1 + 2 = 11
    n_3 = d_4 = 9
    etc.
     */
    let now = Instant::now();
    // GOAL is 99 because I don't count the 2 as part of the sequence
    const GOAL: usize = 99;
    let period = generate_e(GOAL);
    let mut last_n = BigUint::from(0u32);
    let mut n = BigUint::from(1u32);
    let mut d = BigUint::from(1u32);
    for i in (0..GOAL).rev() {
        n = d.clone();
        d *= BigUint::from(period[i]);
        d += last_n.clone();
        last_n = n.clone();
    }
    n += BigUint::from(2u32) * d.clone();
    let mut sum = BigUint::from(0u32);
    while n != BigUint::from(0u32) {
        let digit = n.clone() % BigUint::from(10u32);
        sum += digit;
        n /= BigUint::from(10u32);
    }
    println!("{}", sum);
    println!("{:?}", now.elapsed());
}

fn generate_e(steps: usize) -> Vec<u128> {
    let mut result = vec![1];
    let mut every_third = 2;
    if steps == 1 {
        return result;
    }
    for i in 1..=steps {
        if i % 3 == 1 {
            result.push(every_third);
            every_third += 2;
        } else {
            result.push(1);
        }
    }
    result
}