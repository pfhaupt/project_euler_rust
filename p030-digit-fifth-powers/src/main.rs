// https://projecteuler.net/problem=30

fn main() {
    const POWER: u32 = 5;
    const UPPER_BOUND: u64 = 9u64.pow(POWER) * (POWER - 1) as u64;
    let mut total = 0;
    for n in 2..=UPPER_BOUND {
        let mut sum = 0;
        let mut tmp = n;
        while tmp != 0 {
            let digit = tmp % 10;
            sum += digit.pow(POWER);
            tmp /= 10;
        }
        if sum == n {
            total += n;
        }
    }
    println!("{}", total);
}
