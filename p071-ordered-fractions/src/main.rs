fn main() {
    // https://projecteuler.net/problem=71
    /* 
    The idea is simple:
    Starting at 1/2, evaluate the fraction.
    If it is above the goal, increase the denominator, decrease numerator.
    If it is below the goal, increase the numerator.
    Repeat until d is done.
    This process will converge towards the goal.
    
    From messing around a bit, decreasing the numerator is not necessary, but that'd mean I have to subtract one from my end result.
     */
    const LIMIT: u64 = 1_000_000;
    const GOAL: f64 = 3.0 / 7.0;
    let mut d = 2;
    let mut n = 1;
    while d < LIMIT {
        let frac = n as f64 / d as f64;
        if frac < GOAL {
            n += 1;
        } else {
            n -= 1;
            d += 1;
        }
    }
    println!("{}", n);
}