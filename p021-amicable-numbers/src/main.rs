// https://projecteuler.net/problem=21

use std::time::Instant;

fn main() {
    let now = Instant::now();
    const MAX: usize = 10000;
    let mut sum = 0;
    let mut vals: [usize; MAX] = [0; MAX];
    for n in 0..MAX {
        vals[n] = d(n);
    }
    for n in 0..MAX {
        for m in 0..MAX {
            if vals[n] == m && n == vals[m] && n != m {
                sum += n + m;
            }
        }
    }
    sum /= 2; // The sum contains both (n, m) and (m, n)
    println!("{}", sum);
    println!("{:?}", now.elapsed());
}

fn d(n: usize) -> usize {
    let mut result = 0;
    for i in 1..n {
        if n % i == 0 {
            result += i;
        }
    }
    result
}
