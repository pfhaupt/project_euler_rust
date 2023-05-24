// https://projecteuler.net/problem=62

fn main() {
    let mut old_len = 0;
    let mut len: usize = 10;
    // loop until we hit something
    loop {
        let cubes: Vec<u64> = (0..len).map(|n| (n * n * n) as u64).collect();
        // Nested loops again...
        for a_i in old_len..len {
            let a = &cubes[a_i];
            for b_i in a_i+1..len {
                let b = &cubes[b_i];
                if is_permutation(a, b) {
                    for c_i in b_i+1..len {
                        let c = &cubes[c_i];
                        if is_permutation(b, c) {
                            for d_i in c_i+1..len {
                                let d = &cubes[d_i];
                                if is_permutation(c, d) {
                                    for e_i in d_i+1..len {
                                        let e = &cubes[e_i];
                                        if is_permutation(d, e) {
                                            //println!("{} {} {} {} {}", a, b, c, d, e);
                                            println!("{}", a);
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
        old_len = len;
        len *= 2;
    }
}

fn is_permutation(a: &u64, b: &u64) -> bool {
    let mut d_a = [0; 10];
    let mut t_a = a.clone();
    while t_a != 0 {
        d_a[(t_a % 10) as usize] += 1;
        t_a /= 10;
    }
    let mut d_b = [0; 10];
    let mut t_b = b.clone();
    while t_b != 0 {
        d_b[(t_b % 10) as usize] += 1;
        t_b /= 10;
    }
    for i in 0..10 {
        if d_a[i] != d_b[i] {
            return false;
        }
    }
    true
}