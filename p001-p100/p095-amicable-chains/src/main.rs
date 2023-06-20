// https://projecteuler.net/problem=95

use std::time::Instant;

fn main() {
    let now = Instant::now();
    const MAX: usize = 1_000_000;
    let sqrt_max = f64::sqrt(MAX as f64) as usize;

    // Efficient way of computing divisor sums
    let mut divisor_sum = vec![1; MAX + 1];
    for i in 2..=sqrt_max {
        if i * i < MAX {
            divisor_sum[i * i] += i;
        }
        let mut max = MAX / i;
        if MAX % i != 0 {
            max += 1;
        }
        for k in (i+1)..max {
            divisor_sum[k * i] += k + i;
        }
    }
    
    let mut record_number = 0;
    let mut record_len = 0;
    for n in 1..=MAX {
        // Modified Brent's Algorithm: https://en.wikipedia.org/wiki/Cycle_detection#Brent's_algorithm
        let mut power = 1;
        let mut lam = 1;
        let mut tortoise = n;
        let mut hare = divisor_sum[n];
        while hare != tortoise {
            if hare > MAX {
                // Thing blows up
                lam = 0;
                break;
            }
            if power == lam {
                tortoise = hare;
                power *= 2;
                lam = 0;
            }
            hare = divisor_sum[hare];
            lam += 1;
        }
        tortoise = n;
        hare = n;
        for _ in 0..lam {
            hare = divisor_sum[hare];
        }
        while tortoise != hare {
            tortoise = divisor_sum[tortoise];
            hare = divisor_sum[hare];
        }
        if lam > record_len {
            record_len = lam;
            record_number = hare;
        }
    }

    let mut smallest = usize::MAX;
    for _ in 0..record_len {
        smallest = smallest.min(record_number);
        record_number = divisor_sum[record_number];
    }
    println!("{}", smallest);
    println!("{:?}", now.elapsed());
}