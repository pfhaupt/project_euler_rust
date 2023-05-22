fn main() {
    // https://projecteuler.net/problem=64
    const N: usize = 10_000;

    let mut result = 0;
    for n in 2..=N {
        let period = get_period_length(n as u64);
        if period % 2 == 1 {
            result += 1;
        }
    }
    println!("{}", result);
}

fn get_period_length(n: u64) -> u64 {
    // https://en.wikipedia.org/wiki/Periodic_continued_fraction#Canonical_form_and_repetend
    let a0 = f64::sqrt(n as f64).floor() as u64;
    if a0 * a0 == n {
        // Square number
        return 0;
    }
    let mut m = 0;
    let mut d = 1;
    let mut a = a0;
    let mut counter = 0;
    let mut last = vec![];
    loop {
        m = d * a - m;
        d = (n - m * m) / d;
        a = (a0 + m) / d;
        let triple = (m, d, a);
        if last.contains(&triple) {
            return counter;
        }
        last.push(triple);
        counter += 1;
    }
}