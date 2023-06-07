// https://projecteuler.net/problem=42

use std::fs;
use std::collections::HashSet;
use std::time::Instant;


fn main() {
    let now = Instant::now();
    let triangle_numbers: HashSet<usize> = (1..1000).into_iter().map(|n|n*(n+1)/2).collect();
    let contents = fs::read_to_string("C:/Users/Philippe/RustProjects/project_euler_rust/p042-coded-triangle-numbers/src/words.txt")
        .expect("Something went wrong when loading the file!");
    let names: Vec<&str> = contents.split(",").collect();

    let mut edited: Vec<&str> = vec![];
    for name in names {
        let mut iter = name.chars();
        //Skip the quotes
        iter.next();
        iter.next_back();
        edited.push(iter.as_str());
    }
    let result = edited.iter()
                        .map(|name| is_triangle_word(*name, &triangle_numbers))
                        .filter(|b| *b)
                        .collect::<Vec<bool>>()
                        .len();
    println!("{:?}", result);
    println!("{:?}", now.elapsed());
}

fn is_triangle_word(word: &str, triangle_numbers: &HashSet<usize>) -> bool {
    let chars = word.as_bytes();
    let sum: usize = chars.iter().map(|ch| (*ch as usize) - 64).sum();
    triangle_numbers.contains(&sum)
}