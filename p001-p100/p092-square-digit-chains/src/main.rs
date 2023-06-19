// https://projecteuler.net/problem=92

use std::time::Instant;

fn main() {
    let now = Instant::now();
    const GOAL: usize =  10_000_000;
    let mut visited = vec![0; GOAL];
    for n in 1..GOAL {
        let mut current = n;
        let end = loop {
            if current == 1 || current == 89 {
                break current;
            } else if current < GOAL && visited[current] != 0 {
                break visited[current];
            } else {
                current = square_digits(current);
            }
        };
        visited[n] = end;
    }
    println!("{}", visited.iter().filter(|v|**v == 89).count());
    println!("{:?}", now.elapsed());
}

fn square_digits(n: usize) -> usize {
    let mut n = n;
    let mut result = 0;
    while n != 0 {
        let digit = n % 10;
        result += digit * digit;
        n /= 10;
    }
    result
}
