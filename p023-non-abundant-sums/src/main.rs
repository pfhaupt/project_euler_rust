fn main() {
    const MAX: usize = 100;

    let mut abundant: Vec<usize> = vec![];
    for n in 0..MAX {
        let val = d(n);
        if val > n {
            abundant.push(n);
        }
    }
    let mut hit: [bool; MAX] = [false; MAX];
    for m in &abundant {
        for n in &abundant {
            let sum = m + n;
            if sum >= MAX {
                continue;
            }
            hit[sum] = true;
        }
    }
    
    /* 
    - hit is an array of booleans
    - I create an iterator, to go over that array
    - I enumerate the iterator, to get (index, value) pairs
    - I filter for all valid (index, value) pairs where value==true
    - Of those pairs, I retrieve the indices and store them in an iterator (I think thats what map returns)
    - Apparently I can just call sum() on iterators, which is exactly what I want
     */
    let solution: usize = hit.iter().enumerate().filter(|(_, v)| !(**v)).map(|(i, _)| i).sum();

    // filter()-less cursed version that just sets all invalid elements to 0
    //let solution: usize = hit.iter().enumerate().map(|(i, v)| (!(v) as usize) * i).sum();

    println!("{:?}", solution);
}

fn d(n: usize) -> usize {
    let mut result = 0;
    for i in 1..n {
        if n % i == 0 {
            result += i;
        }
    }
    result
}