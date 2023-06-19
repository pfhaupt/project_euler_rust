// https://projecteuler.net/problem=26

use std::time::Instant;

fn main() {
    let now = Instant::now();
    const MAX: u64 = 1000;
    let period_cycles: Vec<u64> = (0..=MAX).map(|n| get_cycle_len(n)).collect();
    let mut max = 0;
    let mut rec = 0;
    for (index, cycle) in period_cycles.iter().enumerate() {
        if *cycle > max {
            rec = index;
            max = *cycle;
        }
    }
    println!("{}", rec);
    println!("{:?}", now.elapsed());
}

fn get_cycle_len(mut n: u64) -> u64 {
    //https://math.stackexchange.com/questions/377683/length-of-period-of-decimal-expansion-of-a-fraction
    if n == 0 {
        return 0;
    }
    // Having factors of 2 and 5 does not change the cycle length (and doesn't lead to infinite loops)
    while n % 2 == 0 {
        n /= 2;
    }
    while n % 5 == 0 {
        n /= 5;
    }
    if n == 1 {
        return 0;
    }
    let mut pow = 10;
    let mut cycle = 1;
    loop {
        if pow % n == 1 {
            break;
        }
        pow *= 10;
        pow %= n;
        cycle += 1;
    }
    cycle
}