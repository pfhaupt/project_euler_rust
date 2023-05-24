// https://projecteuler.net/problem=9

fn main() {
    const MAX: u32 = 1000;
    for a in 1..=MAX {
        for b in a..=MAX {
            for c in b..=MAX {
                if a * a + b * b == c * c {
                    if a + b + c == MAX {
                        println!("{}", a * b * c);
                        return;
                    }
                }
            }
        }
    }
}
