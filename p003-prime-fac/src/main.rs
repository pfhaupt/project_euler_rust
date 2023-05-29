// https://projecteuler.net/problem=3

fn main() {
    let mut n: i64 = 600851475143;
    let mut biggest_factor = 0;
    while n != 1 {
        let mut factor = 2;
        while n % factor != 0 {
            factor += 1;
        }
        n = n / factor;
        biggest_factor = biggest_factor.max(factor);
    }
    println!("{}", biggest_factor);
}
