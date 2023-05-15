fn main() {
    // https://projecteuler.net/problem=6
    const N: i32 = 100;
    let mut sum = 0;
    let mut prod = 0;
    for n in 1..=N {
        sum += n * n;
        prod += n;
    }
    prod *= prod;
    println!("{}", prod - sum);
}
