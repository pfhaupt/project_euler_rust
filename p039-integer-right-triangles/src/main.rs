// https://projecteuler.net/problem=39

fn main() {
    const MAX: usize = 1000;
    let result: usize = (0..=MAX).into_iter() // generate iterator over all numbers
                            .map(|n| generate_solutions(n).len()) // calculate solutions
                            .enumerate() // transform to index-value pairs
                            .max_by(|(_, v1), (_, v2)| v1.cmp(v2)) // get the index that contains the highest value
                            .unwrap() // unwrap, because it could be empty (I can safely unwrap, because I know it's not, no Some() needed)
                            .0; // get index from pair of values
    println!("{}", result);
}

fn generate_solutions(p: usize) -> Vec<(usize, usize, usize)> {
    let mut result: Vec<(usize, usize, usize)> = vec![];
    for a in 1..p {
        for b in a..p {
            for c in b..p {
                if a * a + b * b == c * c &&
                    a + b + c == p {
                    result.push((a, b, c));
                }
            }
        }
    }
    result
}