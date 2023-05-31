// https://projecteuler.net/problem=22

use std::fs;

fn main() {
    let contents = fs::read_to_string("C:/Users/Philippe/RustProjects/project_euler_rust/p022-names-scores/src/names.txt")
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

    edited.sort();

    let mut sum = 0;
    for (index, name) in edited.iter().enumerate() {
        let score = get_score(name);
        sum += score * (index + 1);
    }
    println!("{}", sum);
}

fn get_score(name: &str) -> usize {
    let mut result = 0;
    for ch in name.chars() {
        result += (ch as usize) - 64;
    }
    result
}
