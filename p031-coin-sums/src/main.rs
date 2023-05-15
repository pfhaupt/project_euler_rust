
const GOAL: u32 = 200;
// UK
// const COURSE: [u32; 8] = [200, 100, 40, 20, 10, 4, 2, 1];
// const WORTH: [u32; 8] = [1, 2, 5, 10, 20, 50, 100, 200];
// US
const COURSE: [u32; 7] = [200, 40, 20, 8, 4, 2, 1];
const WORTH: [u32; 7] = [1, 5, 10, 25, 50, 100, 200];
fn main() {
    let result = check_balance(COURSE.len() as i32 - 1, 0);
    println!("{}", result);
}

fn check_balance(depth: i32, acc: u32) -> u32 {
    if depth == -1 {
        return if acc == GOAL { 1 } else { 0 };
    }
    let depth = depth as usize;
    let mut valid = 0;
    for i in 0..=COURSE[depth] {
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
