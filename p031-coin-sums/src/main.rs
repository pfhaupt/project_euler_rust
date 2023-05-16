
const GOAL: u32 = 200;
// UK
const WORTH: [u32; 8] = [1, 2, 5, 10, 20, 50, 100, 200];
// US
// const WORTH: [u32; 7] = [1, 5, 10, 25, 50, 100, 200];
fn main() {
    // https://projecteuler.net/problem=31
    let result = check_balance(WORTH.len() as i32 - 1, 0);
    println!("{}", result);
}

fn check_balance(depth: i32, acc: u32) -> u32 {
    if depth == -1 {
        return if acc == GOAL { 1 } else { 0 };
    }
    let depth = depth as usize;
    let mut valid = 0;
    let amount = GOAL / &WORTH[depth];
    for i in 0..=amount {
        let this_sum = acc + i * &WORTH[depth];
        if this_sum == GOAL {
            valid += 1;
        } else if this_sum > GOAL {
            break;
        } else {
            valid += check_balance(depth as i32 - 1, this_sum);
        }
    }
    return valid;
}
