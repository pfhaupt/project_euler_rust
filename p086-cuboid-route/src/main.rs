// https://projecteuler.net/problem=86

fn main() {
    /* 
    I've spent the last few hours figuring out some formulas (see calc.txt for more details), but I must've missed something.
    It produces incorrect, albeit close, results, even though it looks correct. I think I made a typo somewhere, or something like that.
    [Revisit 4 hours later] I think my initial claim of "only 3 possibilities because of symmetry" was incorrect.
                            -> I'll have to look at this in the future (if anything, P86 is currently my slowest program), but for now let it stay.
    Well, time for a new approach. Time to uncube the cube.

    z y
    |/
    +-x
       X--------------------F
      /|                   /|
     / |                  / |c
    X--|-----------------X  |
    |  X-----------------|--X
    | /                  | /b
    |/                   |/
    S--------------------X
                a
    Thinking in 3D is actually very hard, now that I think about it.
    When unfolding the cube, we get something like:

        X--------------------X
        |                    |
        |                    |
        |                    |
        X--------------------F
        |                    |
        |                    |c
        |                    |
    X---X--------------------X----X
    |   |                    |    |
    |   |                    |b   |
    |   |                    |    |
    X---S--------------------X----X
        |         a          |
        |                    |
        |                    |
        X--------------------X
    
    ... Yeah this makes things easier.
    SF = sqrt(a * a + (b + c) * (b + c))
    check if SF is an integer, done.
     */

    const GOAL: usize = 1_000_000;
    let mut m = 1;
    let mut solutions = 0;
    loop {
        let a = m;
        for b in 1..=a {
            for c in 1..=b {
                if (f64::sqrt((a * a + (b + c) * (b + c)) as f64)).fract() == 0.0 {
                    solutions += 1;
                }
            }
        }
        if solutions > GOAL {
            println!("{}", m);
            break;
        }
        m += 1;
    }
}
