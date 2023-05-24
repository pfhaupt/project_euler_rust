// https://projecteuler.net/problem=33

use num::integer::gcd;
use std::fmt;

#[derive(Debug)]
struct Fraction {
    numerator: i32,
    denominator: i32
}

impl Fraction {
    fn equals(&self, other: &Fraction) -> bool {
        return self.numerator as f32 / self.denominator as f32 == other.numerator as f32 / other.denominator as f32;
    }
}
impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

fn main() {
    let mut numerator_product = 1;
    let mut denominator_product = 1;

    for numerator in 10..100 {
        for denominator in numerator+1..100 {
            let fraction = Fraction { numerator, denominator };
            let cursed = simplify_incorrect(&fraction);
            let trivial = cursed.0;
            let cursed = cursed.1;
            if fraction.equals(&cursed) {
                if !trivial {
                    let gcd = gcd(numerator, denominator);
                    numerator_product *= numerator / gcd;
                    denominator_product *= denominator / gcd;
                }
            }
        }
    }
    println!("{}", denominator_product / gcd(numerator_product, denominator_product));
}

fn simplify_incorrect(fraction: &Fraction) -> (bool, Fraction) {
    let numerator_str = fraction.numerator.to_string();
    let denominator_str = fraction.denominator.to_string();
    let numerator_digits = numerator_str.as_bytes();
    let denominator_digits = denominator_str.as_bytes();

    // I'll just hardcode the four possibilities for now
    // - 48 because digits-array contains char values
    let d1 = numerator_digits[0] as i32 - 48;
    let d2 = numerator_digits[1] as i32 - 48;
    let d3 = denominator_digits[0] as i32 - 48;
    let d4 = denominator_digits[1] as i32 - 48;
    if d2 == d4 {
        // ones digits are the same
        // Trivial case
        if d2 == 0 {
            return (true, Fraction { numerator: fraction.numerator, denominator: fraction.denominator });
        }
        // ones are not zero, so we can return the "simplified" version
        // 32 / 42 turns into 3 / 4
        return (false, Fraction { numerator: d1, denominator: d3 });
    }
    if d1 == d3 {
        // tens digits are the same
        // 23 / 24 turns into 2 / 3
        return (false, Fraction { numerator: d2, denominator: d4 });
    }
    if d1 == d4 {
        // ten of numerator == one of denominator
        // 23 / 72 turns into 3 / 7
        return (false, Fraction { numerator: d2, denominator: d3 });
    }
    if d2 == d3 {
        // one of numerator == ten of denominator
        // 12 / 27 turns into 1 / 7
        return (false, Fraction { numerator: d1, denominator: d4 });
    }
    // We can't even cursly simplify anything
    (true, Fraction { numerator: fraction.numerator, denominator: fraction.denominator })
}