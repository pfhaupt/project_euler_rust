// https://projecteuler.net/problem=27

fn main() {
    const MAX: i64 = 1000;

    let mut record_a = 0;
    let mut record_b = 0;
    let mut record_n = 0;
    for a in (-MAX+1)..MAX {
        for b in (-MAX)..=MAX {
            let mut n = 0;
            loop {
                let result = n * n + a * n + b;
                if !is_prime(result) {
                    break;
                }
                n += 1;
            }
            if n > record_n {
                record_n = n;
                record_a = a;
                record_b = b;
            }
        }
    }
    println!("{}", record_a * record_b);
}

fn is_prime(n: i64) -> bool {
    if n < 0 {
        return false;
    }
    if n == 1 || n == 0 {
        return false;
    }
    if n == 2 {
        return true;
    }
    let sqrt = f64::sqrt(n as f64) as i64;
    for i in 2..=sqrt {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}
