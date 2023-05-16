fn main() {
    // https://projecteuler.net/problem=41
    println!("Hello, world!");
    let mut n: usize = 1;
    let mut biggest = 0;
    loop {
        if n == 9 {
            // we can stop at 8 digits, because all 9-digit pandigitals are divisible by 3.
            break;
        }
        let primes: Vec<usize> = calc_primes(10usize.pow(n as u32));
        let pandigital_primes: Vec<usize> = primes.into_iter().filter(|p| is_pandigital(*p, n)).collect();
        if let Some(record) = pandigital_primes.into_iter().max() { // First time ever needing Some()
            biggest = record; // because our digits always increase, we don't need max()
            println!("{} is the current record for {}-digits", record, n);
        } else {
            println!("No pandigital prime for {}-digits", n);
        }
        n += 1;
    }
    println!("{}", biggest);
}

fn is_pandigital(number: usize, n: usize) -> bool {
    let number = number.to_string();
    for digit in 1..=n {
        let s = digit.to_string();
        if !number.contains(&s) {
            return false;
        }
    }
    number.len() == n
}

fn calc_primes(max: usize) -> Vec<usize> {
    let mut primes: Vec<usize> = vec![2];
    for number in (3..=max).step_by(2) {
        if is_prime(number, &primes) {
            primes.push(number);
        }
    }
    primes
}

fn is_prime(number: usize, primes: &Vec<usize>) -> bool {
    let sqrt = f64::sqrt(number as f64) as usize;
    for prime in primes {
        if prime > &sqrt {
            break;
        }
        if number % prime == 0 {
            return false
        }
    }
    true
}