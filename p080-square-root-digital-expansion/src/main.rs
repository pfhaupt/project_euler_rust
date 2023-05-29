// https://projecteuler.net/problem=80

use num_bigint::BigUint;

fn main() {
    let mut total = 0;
    for n in 1..100 {
        let expansion = calculate_digits(n, 99);
        let sum = get_sum(&expansion);
        if sum * sum != n {
            // n is square number if sum * sum == n
            total += sum;
        }
    }
    println!("{}", total);
}

fn get_sum(expansion: &BigUint) -> u64 {
    let mut expansion = expansion.clone();
    let mut sum = 0;
    while &expansion != &BigUint::from(0u64) {
        let digit = (&expansion % BigUint::from(10u64)).to_u64_digits();
        if digit.len() > 0 {
            // vector is empty if there's a 0 at that digit
            sum += digit[0];
        }
        expansion /= BigUint::from(10u64);
    }
    sum
}

fn calculate_digits(n: u64, count: u32) -> BigUint {
    /*
    https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Heron's_method
    Because I haven't found a BigDecimal library yet (well, I haven't looked for one yet), I'll need to 'cheat' a bit:
    We're working with Integers, so we just shift everything by many powers of 10.
    if x = sqrt(S) and S = offset*n, then x = sqrt(offset * n) = sqrt(offset) * sqrt(n)
    With this approach, we're getting log10(sqrt(offset)) digits.
    The *2 in the line below is just to adjust for that sqrt: offset = 10^(count * 2) = (10^count)^2
    => We get log10(sqrt((10^count)^2)) = log10(10^count) = count digits.
     */
    let offset = BigUint::from(10u64).pow(count * 2);
    let s = BigUint::from(n) * &offset;
    let mut x = BigUint::from(n) * &offset;
    for _ in 0.. {
        let last_x = x.clone();
        x = (&x + &s / &x) / BigUint::from(2u32);
        if x == last_x {
            break;
        }
    }
    x
}