fn main() {
    // https://projecteuler.net/problem=72
    // Idea: https://en.wikipedia.org/wiki/Farey_sequence#Sequence_length_and_index_of_a_fraction
    const D: u64 = 1_000_000;
    let mut farey_values = vec![0; D as usize + 1];
    let result = farey(D, &mut farey_values) - 2;
    println!("{}", result);
}

fn farey(n: u64, already_calculated: &mut Vec<u64>) -> u64 {
    if n == 1 {
        already_calculated[1] = 2;
        2
    } else if already_calculated[n as usize] != 0 {
        already_calculated[n as usize]
    } else {
        let mut result = n * (n + 3) / 2;
        for d in 2..=n {
            result -= farey(n / d, already_calculated);
        }
        already_calculated[n as usize] = result;
        result
    }
}