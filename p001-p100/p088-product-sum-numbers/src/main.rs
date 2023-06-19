// https://projecteuler.net/problem=88

use std::{collections::HashSet};
use std::time::Instant;

const GOAL: usize = 12_000;

fn main() {
    let now = Instant::now();
    let mut set = HashSet::new();
    let limit = f64::log2(GOAL as f64) as usize + 1;
    set.extend(calculate_all(limit));
    println!("{}", set.iter().sum::<usize>());
    println!("{:?}", now.elapsed());
}

fn calculate_all(limit: usize) -> Vec<usize> {
    let mut best_so_far = vec![usize::MAX; GOAL + 1];
    for len in 2..=limit {
        let combs = find_combinations(len);
        for numbers in combs {
            let prod = get_product(&numbers);
            let sum = get_sum(&numbers);
            if prod >= sum {
                let diff = prod - sum;
                let k = len + diff;
                if k <= GOAL {
                    best_so_far[k] = best_so_far[k].min(prod);
                }
            }
        }
    }
    let mut result = vec![];
    for b in best_so_far {
        if b != usize::MAX {
            result.push(b);
        }
    }
    result
}

fn recursive_helper(numbers: &mut Vec<usize>, index: usize, acc: &mut Vec<Vec<usize>>) {
    if index >= numbers.len() {
        return;
    }
    while get_product(numbers) <= 2 * GOAL {
        numbers[index] += 1;
        acc.push(numbers.clone());
        recursive_helper(numbers, index + 1, acc);
    }
    
    numbers[index] = if index == 0 { 1 } else { numbers[index - 1] - 1 };
}

fn find_combinations(len: usize) -> Vec<Vec<usize>> {
    let mut result = vec![];
    let mut numbers = vec![1; len];
    recursive_helper(&mut numbers, 0, &mut result);
    result
}

fn get_sum(numbers: &Vec<usize>) -> usize {
    let mut sum = 0;
    for &n in numbers {
        sum += n;
    }
    sum
}

fn get_product(numbers: &Vec<usize>) -> usize {
    let mut product = 1;
    for &n in numbers {
        product *= n;
    }
    product
}