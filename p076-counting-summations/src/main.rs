// https://projecteuler.net/problem=76

use std::time::Instant;

const GOAL: usize = 101;

fn main() {
    /*
    https://oeis.org/A000065
    https://oeis.org/A026820

    Recursively:
    fn T(n: usize, k: usize) -> usize {
        if n == 0 || k == 1 {
            1
        } else {
            T(n, k - 1) + if k > n { 0 } else { T(n - k, k) }
        }
    }
    (Not feasible calling T(100, 100))
    instead: dynamic programming
    */
    let now = Instant::now();
    let mut triangle = vec![vec![0; GOAL]; GOAL];
    for n in 0..GOAL {
        for k in 1..GOAL {
            triangle[n][k] = if n == 0 || k == 1 {
                1
            } else {
                triangle[n][k - 1] + if k > n { 0 } else { triangle[n - k][k] }
            };
        }
    }
    println!("{}", triangle[GOAL - 1][GOAL - 2]);
    println!("{:?}", now.elapsed());
}