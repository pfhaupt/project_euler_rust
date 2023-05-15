fn main() {
    // https://projecteuler.net/problem=14
    let mut record = 0;
    let mut number = 0;
    const MAX: u64 = 1_000_000;
    let mut n = 1;

    while n < MAX {
        let steps = collatz(n);
        if steps > record {
            record = steps;
            number = n;
        }
        n += 1;
    }
    println!("{}", number);
}

fn collatz(n: u64) -> u64 {
    let mut steps = 0;
    let mut current = n;
    while current != 1 {
        if current % 2 == 0 {
            current = current / 2;
        } else {
            current = 3 * current + 1;
        }
        steps += 1;
    }
    steps
}
