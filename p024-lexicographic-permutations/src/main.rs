use itertools::Itertools;

fn main() {
    const NEEDED: usize = 1_000_000 - 1;
    let numbers = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    if let Some(solution) = numbers.iter().permutations(numbers.len()).nth(NEEDED) {
        println!("{:?}", solution);
        let mut result = String::from("");
        for elem in solution {
            result += &elem.to_string();
        }
        println!("{}", result);
    } else {
        panic!("\nThere is no {NEEDED}th permutation of the set {numbers:?}!\n");
    }
}
