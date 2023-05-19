use itertools::Itertools;

fn main() {
    // https://projecteuler.net/problem=61
    let mut polygons: Vec<Vec<i64>> = vec![];
    for s in 3..=8 {
        let mut n = 1;
        let mut poly = vec![];
        loop {
            let val = p(s, n);
            if val > 10000 {
                break;
            } else if val > 1000 && val % 100 > 9 {
                poly.push(val);
            }
            n += 1;
        }
        polygons.push(poly);
    }
    // Let's just ignore this behemoth
    // I couldn't find a better solution than basically brute-force
    // This code is not flexible at all, but hey - All I care about is the result for 6, I guess
    // But this screams for recursion...
    // Basically I create all possible permutations for arranging 6 polygons in the list
    // Then check for all those polygons, if they are in a cycle with the next
    let perms = polygons.iter().permutations(6);
    for p in perms {
        for &a in p[0] {
            for &b in p[1] {
                if is_cyclic(a, b) {
                    for &c in p[2] {
                        if is_cyclic(b, c) {
                            for &d in p[3] {
                                if is_cyclic(c, d) {
                                    for &e in p[4] {
                                        if is_cyclic(d, e) {
                                            for &f in p[5] {
                                                if is_cyclic(e, f) {
                                                    if is_cyclic(f, a) {
                                                        println!("{} {} {} {} {} {}", a, b, c, d, e, f);
                                                        println!("{}", a + b + c + d + e + f);
                                                        return;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn p(s: i64, n: i64) -> i64 {
    ((s - 2) * n * n - (s - 4) * n) / 2
}


fn is_cyclic(first: i64, second: i64) -> bool {
    first / 100 == second % 100
}