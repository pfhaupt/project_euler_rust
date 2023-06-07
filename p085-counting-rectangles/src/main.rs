// https://projecteuler.net/problem=85

use std::time::Instant;

fn main() {
    let now = Instant::now();
    const GOAL: usize = 2_000_000;
    const UPPER_BOUND: usize = 2_000; // a 2000x1 rectangle already has more subrectangles than the GOAL requires
    let mut closest_bet = usize::MAX;
    let mut closest_area = 0;

    for width in 1..=UPPER_BOUND {
        for height in width..=UPPER_BOUND { // height can go from width because of the symmetric property of the formula -> f(x, y) = f(y, x)
            // let mut sum = 0;
            // for w in 1..=width {
            //     for h in 1..=height {
            //         let small_rect_count = (width - w + 1) * (height - h + 1);
            //         sum += small_rect_count;
            //     }
            // }
            /* 
            c = (W - w + 1) * (H - h + 1) for w in 1..=W, h in 1..=H
            c = sum(H - h + 1) * sum(W - w + 1)

            c = s_h * s_w
            s_h = sum(H - h + 1) for h in 1..=H
            s_h = sum(H) - sum(h) + sum(1)
            s_h = H * H - sum(h) + H
            s_h = H * H + H - sum(h)
            s_h = H * H + H - (H * (H + 1) / 2)
            s_w = W * W + W - (W * (W + 1) / 2)
            
            s_h = H * H + H - (H * H + H) / 2
            s_h = (H * H + H) / 2
            s_h = H * (H + 1) / 2
            s_w = W * (W + 1) / 2
             */
            let s_h = height * (height + 1) / 2;
            let s_w = width * (width + 1) / 2;
            let sum = s_w * s_h;
            let abs_diff = GOAL.abs_diff(sum);
            if abs_diff < closest_bet {
                closest_bet = abs_diff;
                closest_area = width * height;
            }
        }
    }
    println!("{}", closest_area);
    println!("{:?}", now.elapsed());
}
