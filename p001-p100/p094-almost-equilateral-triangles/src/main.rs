// https://projecteuler.net/problem=94

use std::time::Instant;

fn main() {
    // https://en.wikipedia.org/wiki/Integer_triangle#Isosceles_Heronian_triangles
    // Bruteforce approach, just try all possible perimeters, and add up all solutions we find along the way

    let now = Instant::now();
    // const MAX: u64 = 1_000_000_000;
    // let mut sum = 16; // I don't check (5, 5, 6)
    // let mut perimeter = 18;
    // while perimeter < MAX {
    //     // In this loop we're going over all equilateral triangles (perimeter % 3 == 0 and 3 * a = perimeter), and generate side length candidates on the way
    //     // get the length of the two equal sides
    //     let a = perimeter / 3;
    //     // |a - b| = 1, so just try both cases.
    //     let b = a - 1;
    //     if valid(a, b) {
    //         sum += perimeter - 1; // 2 * a + (b - 1) = 2 * a + b - 1 = perimeter - 1
    //         perimeter *= 3; // the ratio of neighboring triangles is a bit more (around 3.75 from printing a few values), but I am working with ints
    //         continue; // We don't need to check b = a + 1 anymore and can save a sqrt
    //     }
    //     let b = a + 1;
    //     if valid(a, b) {
    //         sum += perimeter + 1; // 2 * a + (b + 1) = 2 * a + b + 1 = perimeter + 1
    //         perimeter *= 3; // the ratio of neighboring triangles is a bit more (around 3.75 from printing a few values), but I am working with ints
    //     }
    //     perimeter += 3;
    // }

    // https://oeis.org/A120893
    const MAX: i64 = 1_000_000_000;
    let mut sum = 0;
    let (mut a1, mut a2, mut a3) = (1, 1, 1);
    let mut perimeter = 0;
    let mut minus_sign = 1;
    while perimeter < MAX {
        sum += perimeter;
        let a = 3 * a1 + 3 * a2 - a3;
        let c = (a + minus_sign) / 2;
        perimeter = 2 * (a + c);
        a3 = a2;
        a2 = a1;
        a1 = a;
        minus_sign *= -1;
    }
    println!("{}", sum);
    println!("{:?}", now.elapsed());
}

#[allow(dead_code)]
fn valid(a: u64, b: u64) -> bool {
    let c = 4 * a * a - b * b;
    let s = f64::sqrt(c as f64) as u64;
    s * s == c
}
