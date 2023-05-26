// https://projecteuler.net/problem=76

const GOAL: i32 = 100;

fn main() {
    let result = count_ways(GOAL, 0) - 1;
    println!("{}", result);
}

fn count_ways(depth: i32, acc: i32) -> i32 {
    // Adapted from Problem 31
    if depth == 0 {
        return if acc == GOAL { 1 } else { 0 };
    }
    let mut valid = 0;
    let amount = GOAL / depth;
    for i in 0..=amount {
        let this_sum = acc + i * depth;
        if this_sum == GOAL {
            valid += 1;
        } else if this_sum > GOAL {
            break;
        } else {
            valid += count_ways(depth - 1, this_sum);
        }
    }
    return valid;
}
