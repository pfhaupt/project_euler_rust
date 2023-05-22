use std::{ops, fmt};
use num_bigint::BigInt;

#[derive(Clone, Copy)]
struct Fraction {
    numerator: u128,
    denominator: u128
}

impl Fraction {
    fn simplify(&self) -> Fraction {
        *self
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

impl ops::Add<Fraction> for Fraction {
    type Output = Fraction;

    fn add(self, rhs: Fraction) -> Fraction {
        if self.denominator == 0 || rhs.denominator == 0 {
            panic!("Denominator of a number is zero!");
        }
        if self.numerator == 0 {
            return rhs;
        } else if rhs.numerator == 0 {
            return self;
        }
        let lcm = lcm(self.denominator, rhs.denominator);
        let scaling_lhs = lcm / self.denominator;
        let scaling_rhs = lcm / rhs.denominator;
        let n1 = self.numerator * scaling_lhs;
        let d1 = self.denominator * scaling_lhs;
        let n2 = rhs.numerator * scaling_rhs;
        let d2 = rhs.denominator * scaling_rhs;
        assert_eq!(d1, d2);
        Fraction { numerator: n1 + n2, denominator: d1 }.simplify()
    }
}

impl ops::Sub<Fraction> for Fraction {
    type Output = Fraction;

    fn sub(self, rhs: Fraction) -> Fraction {
        if self.denominator == 0 || rhs.denominator == 0 {
            panic!("Denominator of a number is zero!");
        }
        if self.numerator == 0 {
            return rhs;
        } else if rhs.numerator == 0 {
            return self;
        }
        let lcm = lcm(self.denominator, rhs.denominator);
        let scaling_lhs = lcm / self.denominator;
        let scaling_rhs = lcm / rhs.denominator;
        let n1 = self.numerator * scaling_lhs;
        let d1 = self.denominator * scaling_lhs;
        let n2 = rhs.numerator * scaling_rhs;
        let d2 = rhs.denominator * scaling_rhs;
        assert_eq!(d1, d2);
        Fraction { numerator: n1 - n2, denominator: d1 }.simplify()
    }
}

impl ops::Div<Fraction> for Fraction {
    type Output = Fraction;

    fn div(self, rhs: Fraction) -> Fraction {
        let reciprocal = Fraction { numerator: rhs.denominator, denominator: rhs.numerator };
        (self * reciprocal).simplify()
    }
}

impl ops::Mul<Fraction> for Fraction {
    type Output = Fraction;

    fn mul(self, rhs: Fraction) -> Fraction {
        let n = self.numerator * rhs.numerator;
        let d = self.denominator * rhs.denominator;
        Fraction { numerator: n, denominator: d }.simplify()
    }
}

fn lcm(x: u128, y: u128) -> u128 {
    if x == y { x } else { (x * y) / gcd(x, y) }
}

fn gcd(mut x: u128, mut y: u128) -> u128 {
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

fn main() {
    // https://projecteuler.net/problem=66
    /* 
    https://en.wikipedia.org/wiki/Diophantine_equation
    https://en.wikipedia.org/wiki/Pells_equation
    For Pells equation we generate the continued fraction of sqrt(d).
    This requires the use of a struct that keeps numerator and denominator apart. (The first 100 lines of this project)
    We then evaluate the fraction, one step at the time, until we find a solution that satisfies the diophantine equation.
    Once we found that solution, we can stop, because Pells equation finds the fundamental solution.
     */
    let mut d_record = 0;
    let mut x_record = 0;
    const D_LIMIT: u128 = 1000;
    for d in 0..=D_LIMIT {
        let mut fraction = generate_fraction(d, D_LIMIT);
        if fraction.len() == 0 {
            continue;
        }
        let offset = Fraction { numerator: fraction.remove(0), denominator: 1 };
        let fraction_as_struct = fraction.iter().map(|f| Fraction { numerator: *f, denominator: 1 }).collect::<Vec<Fraction>>();
        for i in 0..fraction_as_struct.len() {
            let a = offset + recursive_fraction(&fraction_as_struct, 0, i);
            if is_diophantine(a.numerator, a.denominator, d) {
                if a.numerator > x_record {
                    d_record = d;
                    x_record = a.numerator;
                }
                break;
            }
        }
    }
    println!("{}", d_record);
}

fn is_diophantine(x: u128, y: u128, d: u128) -> bool {
    // y^2 and x^2 can get very big, we need to use BigInts again
    let x = BigInt::from(x);
    let y = BigInt::from(y);
    let d = BigInt::from(d);
    &x * &x - d * &y * &y == BigInt::from(1u32)
}

fn generate_fraction(n: u128, steps: u128) -> Vec<u128> {
    // https://math.stackexchange.com/q/4425617
    let sn = f64::sqrt(n as f64) as u128;
    if sn * sn == n {
        return vec![];
    }
    let mut a;
    let mut r = 0;
    let mut s = 1;
    let mut fraction = vec![];
    for _ in 0..steps {
        a = (r + sn) / s;
        r = a * s - r;
        s = (n - r * r) / s;
        fraction.push(a);
    }
    fraction
}

fn recursive_fraction(fraction: &Vec<Fraction>, index: usize, stop: usize) -> Fraction {
    if index >= stop {
        Fraction { numerator: 0, denominator: 1 }
    } else {
        Fraction { numerator: 1, denominator: 1 } / (fraction[index] + recursive_fraction(fraction, index + 1, stop))
    }
}