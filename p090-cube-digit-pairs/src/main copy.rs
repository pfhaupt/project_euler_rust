// https://projecteuler.net/problem=90

use std::time::Instant;

fn main() {
    let now = Instant::now();
    let arrangements = get_arrangements();
    let squares = (1..10).map(|x|x*x).collect::<Vec<u8>>();
    println!("{:?}", squares);
    println!("{}", arrangements.len());
    let mut valid = 0;
    for i in 0..arrangements.len() {
        let arr1 = &arrangements[i];
        for j in (i+1)..arrangements.len() {
            let arr2 = &arrangements[j];
            if is_hit(&arr1, &arr2, &squares) {
                println!("{:?}, {:?}", arr1, arr2);
                valid += 1;
            }
        }
    }
    println!("{}", valid);
    println!("{:?}", now.elapsed());
}

fn is_hit(arr1: &Vec<u8>, arr2: &Vec<u8>, squares: &Vec<u8>) -> bool {
    let len = arr1.len();
    let mut hit = vec![false; squares.len()];
    for i in 0..len {
        let d1 = arr1[i];
        for j in 0..len {
            let d2 = arr2[j];
            for s_i in 0..squares.len() {
                let s = squares[s_i];
                let s1 = s % 10;
                let s2 = s / 10;
                if (d1 == s1 && d2 == s2) || (d1 == s2 && d2 == s1) {
                    hit[s_i] = true;
                }
                if d1 == 6 {
                    if (9 == s1 && d2 == s2) || (9 == s2 && d2 == s1) {
                        hit[s_i] = true;
                    }
                } else if d2 == 6 {
                    if (d1 == s1 && 9 == s2) || (d1 == s2 && 9 == s1) {
                        hit[s_i] = true;
                    }
                }
            }
        }
    }
    hit.iter().all(|v| *v)
}

fn get_arrangements() -> Vec<Vec<u8>> {
    let mut result = vec![];
    for a in 0..9 {
        for b in (a+1)..9 {
            for c in (b+1)..9 {
                for d in (c+1)..9 {
                    for e in (d+1)..9 {
                        for f in (e+1)..9 {
                            result.push(vec![a, b, c, d, e, f]);
                        }
                    }
                }
            }
        }
    }
    result
}