// https://projecteuler.net/problem=58

fn main() {
    /* 
    37 36 35 34 33 32 31
    38 17 16 15 14 13 30
    39 18  5  4  3 12 29
    40 19  6  1  2 11 28
    41 20  7  8  9 10 27
    42 21 22 23 24 25 26
    43 44 45 46 47 48 49
    size:         1 3  5  7                                  7  5 3
    top-right:    1 3 13 31 // (size - 2)^2 + (size - 1)    18 10 2
    top-left:     1 5 17 37 // (size - 1)^2 + 1             20 12 4
    bottom-left:  1 7 21 43 // size^2 - size + 1            22 14 6
    bottom-right: 1 9 25 49 // always square, do not care   24 16 8
    TOP-RIGHT:
    d = (s2 - 2)^2 + (s2 - 1) - ((s1 - 2)^2 + (s1 - 1)
    s2^2 - 4s2 + 4 + s2 - 1 - (s1^2 - 4s1 + 4 + s1 - 1)
    s2 = s1 + 2
    s2^2 - 3s2 + 3 - (s1^2 - 3s1 + 3)
    (s1 + 2)^2 - 3 * (s1 + 2) + 3 - (s1^2 - 3s1 + 3)
    4 * s1 - 2
    4 * (s2 - 2) - 2
    d = 4 * s - 10
     */
    let mut size = 3;
    let mut prime_counter = 0;
    let mut numbers_on_diagonals = 1;
    let mut v1 = 1;
    let mut v2 = 1;
    let mut v3 = 1;
    let mut _v4 = 1;
    loop {
        // let v1 = (size - 2) * (size - 2) + (size - 1);
        // let v2 = (size - 1) * (size - 1) + 1;
        // let v3 = size * size - size + 1;
        // let _v4 = size * size; // Last element in our square, never prime
        let d1 = 4 * size - 10;
        let d2 = d1 + 2;
        let d3 = d1 + 4;
        let _d4 = d1 + 6;
        v1 += d1;
        v2 += d2;
        v3 += d3;
        _v4 += _d4;
        if is_prime(v1) {
            prime_counter += 1;
        }
        if is_prime(v2) {
            prime_counter += 1;
        }
        if is_prime(v3) {
            prime_counter += 1;
        }
        numbers_on_diagonals += 4;
        let ratio = prime_counter as f64 / numbers_on_diagonals as f64;
        
        if ratio < 0.1 {
            break;
        }
        size += 2;
    }
    println!("{}", size);
}

fn is_prime(candidate: u64) -> bool {
    if candidate == 1 || candidate % 2 == 0 {
        return false;
    }
    let sqrt = f64::sqrt(candidate as f64) as u64;
    for n in (3..=sqrt).step_by(2) {
        if candidate % n == 0 {
            return false;
        }
    }
    true
}
