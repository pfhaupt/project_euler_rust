// https://projecteuler.net/problem=78

#![allow(dead_code)]

use num_bigint::BigInt;

fn main() {
    /*
    https://en.wikipedia.org/wiki/Partition_function_%28number_theory%29#Recurrence_relations
    This is the first problem that I can't solve in under one minute, even in release mode.
    Even with memoization this is pretty slow - Probably because of BigInt multiplication and d(n), which has a complexity of O(n).
    ...
    Even after implementing the 2nd formula, I am still not happy. It's a bit faster (10%, benchmarked up to n=25_000) and doesn't need to compute divisors, but still.
    Some people on the forum are getting times in the milliseconds... 🤔 But I don't understand where their algorithms are coming from.
    Well, I'll revisit this later.
    My best bet is cloning all those values is not very efficient. But isn't that what you're supposed to do?
    Just kidding, you're not.
    Fixed that, and now p2(n) calculates up to n=10_000 in 2 seconds. Not bad, but still not perfect..
    48 seconds in release mode to get the solution - The rule is happy.
    Modulo magic reduced the time down to 9 seconds in release - I do not need to use BigInt anymore!
    70 seconds without release.. I think I'll have to take it.
    [Revisit] I have found a very obvious optimization in p3:
              If I know that n - k_i is already lower than zero, I don't need to check n - k_(i+1), n - k_(i+2) etc.
              This brings the time down to 0.79 seconds without release mode. Nice.
     */
    let size = 100_000;
    // let pre_computed_p = vec![BigInt::from(0); size];
    // let pre_computed_d = vec![BigInt::from(0); size];
    let mut pre_computed_p = vec![0; size];

    let mut n = 1;
    loop {
        // let result = p1(n, &mut pre_computed_p, &mut pre_computed_d);
        // let result = p2(n, &mut pre_computed_p);
        let result = p3(n, &mut pre_computed_p);
        if result == 0 {
            println!("{}", n);
            return;
        }
        // if &result % BigInt::from(1_000_000) == BigInt::from(0) {
        //     println!("{} {}", n, result);
        //     println!("{:?}", now.elapsed());
        //     return;
        // }
        n += 1;
    }
}

fn p3(n: i64, pre_computed_p: &mut Vec<i64>) -> i64 {
    if n < 0 { return 0; }
    else if n == 0 { return 1; }
    else if pre_computed_p[n as usize] != 0 { return pre_computed_p[n as usize]; }
    let mut sum = 0;
    let mut minus_one_flip = 1;
    for k in 1..=n {
        let mut left_side_valid = true;
        let mut right_side_valid = true;
        let mut summand = 0;
        let left_k = k * (3 * k - 1) / 2;
        summand += p3(n - left_k, pre_computed_p);
        if left_k >= n {
            left_side_valid = false;
        }
        let right_k = k * (3 * k + 1) / 2;
        summand += p3(n - right_k, pre_computed_p);
        if right_k >= n {
            right_side_valid = false;
        }
        sum += minus_one_flip * summand;
        sum %= 1_000_000;
        minus_one_flip *= -1;
        if !left_side_valid && !right_side_valid {
            break;
        }
    }
    pre_computed_p[n as usize] = sum;
    sum
}

fn p2(n: i64, pre_computed_p: &mut Vec<BigInt>) -> BigInt {
    if n < 0 { return BigInt::from(0); }
    if n == 0 { return BigInt::from(1); }
    if pre_computed_p[n as usize] != BigInt::from(0) { return pre_computed_p[n as usize].clone(); }
    let mut sum = BigInt::from(0);
    let mut minus_one_flip = BigInt::from(1);
    let minus_one = BigInt::from(-1);
    for k in 1..=n {
        sum = sum + &minus_one_flip * (p2(n - k * (3 * k - 1) / 2, pre_computed_p) + p2(n - k * (3 * k + 1) / 2, pre_computed_p));
        minus_one_flip *= &minus_one;
    }
    pre_computed_p[n as usize] = sum.clone();
    sum
}

fn p1(n: i64, pre_computed_p: &mut Vec<BigInt>, pre_computed_d: &mut Vec<BigInt>) -> BigInt {
    if n < 0 { return BigInt::from(0); }
    if n == 0 { return BigInt::from(1); }
    if pre_computed_p[n as usize] != BigInt::from(0) { return pre_computed_p[n as usize].clone(); }
    let mut sum = BigInt::from(0);
    for k in 0..n {
        sum += d(n - k, pre_computed_d) * p1(k, pre_computed_p , pre_computed_d);
    }
    sum /= BigInt::from(n);
    pre_computed_p[n as usize] = sum.clone();
    sum
}

fn d(n: i64, pre_computed_d: &mut Vec<BigInt>) -> BigInt {
    if pre_computed_d[n as usize] != BigInt::from(0) { return pre_computed_d[n as usize].clone(); }
    let mut result = BigInt::from(0);
    for i in 1..=n {
        if n % i == 0 {
            result += BigInt::from(i);
        }
    }
    pre_computed_d[n as usize] = result.clone();
    result
}