// https://projecteuler.net/problem=75

fn main() {
    const LIMIT: usize = 1_500_000;
    let mut hits = vec![0; LIMIT + 1];
    let sqrt_limit = f64::sqrt(LIMIT as f64) as usize + 1;
    // https://en.wikipedia.org/wiki/Pythagorean_triple#Generating_a_triple
    for n in 1..=sqrt_limit {
        for m in ((n + 1)..=sqrt_limit).step_by(2) {
            if gcd(n, m) != 1 {
                continue;
            }
            let a = m * m - n * n;
            let b = 2 * m * n;
            let c = m * m + n * n;
            let total = a + b + c;
            for index in (total..=LIMIT).step_by(total) {
                hits[index] += 1;
            }
        }
    }
    let result: usize = hits.iter().filter(|&val|*val == 1).sum();
    println!("{}", result);
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}