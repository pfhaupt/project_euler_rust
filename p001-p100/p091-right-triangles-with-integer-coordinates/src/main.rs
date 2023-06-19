// https://projecteuler.net/problem=91

use std::time::Instant;
use std::collections::HashSet;

fn main() {
    let now = Instant::now();
    const GOAL: i32 = 50;
    let mut solutions = HashSet::new();
    for x1 in 0..=GOAL {
        for y1 in 0..=GOAL {
            if x1 == 0 && y1 == 0 {
                continue;
            }
            for x2 in 0..=GOAL {
                for y2 in 0..=GOAL {
                    if (x2 == 0 && y2 == 0) || (x1 == x2 && y1 == y2) {
                        continue;
                    }
                    let a = x1 * x1 + y1 * y1;
                    let b = x2 * x2 + y2 * y2;
                    let dx = x2 - x1;
                    let dy = y2 - y1;
                    let c = dx * dx + dy * dy;
                    let d1 = a.min(b.min(c));
                    let d3 = a.max(b.max(c));
                    let d2 = (a + b + c) - (d1 + d3);
                    if d1 + d2 == d3 {
                        let h1 = x1 * 100 + y1;
                        let h2 = x2 * 100 + y2;
                        let h = h1.min(h2) * 10000 + h1.max(h2);
                        if !solutions.contains(&h) {
                            solutions.insert(h);
                        }
                    }
                }
            }
        }
    }
    println!("{}", solutions.len());
    println!("{:?}", now.elapsed());
}
