// https://projecteuler.net/problem=88

#![allow(dead_code)]

use std::{collections::HashSet};
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let mut set = HashSet::new();
    const GOAL: usize = 12_000;
    for k in 2..=GOAL {
        let result = calculate_number(k);
        set.insert(result);
    }
    println!("{}", set.iter().sum::<usize>());
    println!("{:?}", now.elapsed());
}

fn calculate_number(k: usize) -> usize {
    let mut numbers = vec![1; k];
    let log2 = f64::log2(k as f64) as usize + 1;
    return other_way(&mut numbers, log2);
    // // println!("Calculating limit for k={k} with log2={log2}");
    // let mut increment_number = 1;
    // let mut decrement_number = 2;
    // loop {
    //     let sum = get_sum(&numbers);
    //     let prod = get_product(&numbers);
    //     println!("{:?} {} {}", numbers, prod, sum);
    //     if prod < sum {
    //         increase(&mut numbers, &mut increment_number, log2);
    //     } else if prod > sum {
    //         // todo!("Actual decrement");
    //         decrease(&mut numbers, &mut decrement_number, log2);
    //     } else {
    //         break sum;
    //     }
    // }
}

fn recursive(numbers: &mut Vec<usize>, index: usize, limit: usize, other_limit: usize) -> Result<usize, &str> {
    let mut result = usize::MAX;
    let prod = get_product(numbers);
    let sum = get_sum(numbers);
    if prod > sum {
        return Err("Product too big.");
    } else if sum == prod {
        return Ok(sum);
    } else if index > limit || index >= numbers.len() {
        return Err("Limit reached");
    } else if index == 0 {
        for d in 2..other_limit {
            numbers[index] = d;
            let next_step = recursive(numbers, index + 1, limit, other_limit);
            if next_step.is_ok() {
                result = result.min(next_step.unwrap());
            }
            numbers[index] = 1;
        }
    } else {
        for d in 2..other_limit {
            if d > numbers[index - 1] {
                break;
            }
            numbers[index] = d;
            let next_step = recursive(numbers, index + 1, limit, other_limit);
            if let Err("Product too big.") = next_step {
                numbers[index] = 1;
                break;
            } else {
                result = result.min(next_step.unwrap());
            }
            numbers[index] = 1;
        }
    }
    Ok(result)
}

fn other_way(numbers: &mut Vec<usize>, limit: usize) -> usize {
    return recursive(numbers, 0, limit, numbers.len() + 1).unwrap();
}

fn increase(numbers: &mut Vec<usize>, current: &mut usize, limit: usize) {
    if *current > numbers[0] {
        panic!("This should never happen: Need to increment too big of a value {current}.");
    }
    let index = find_first(&numbers, *current, limit);
    // println!("INCREASING {:?} {}", numbers, current);
    if index == numbers.len() {
        *current += 1;
        increase(numbers, current, limit);
    } else {
        numbers[index as usize] += 1;
    }
}

fn decrease(numbers: &mut Vec<usize>, current: &mut usize, limit: usize) {
    if *current > numbers[0] {
        panic!("This should never happen: Need to decrement too big of a value.");
    }
    let index = find_last(&numbers, *current, limit);
    // println!("DECREASING {:?} {}", numbers, current);
    if index == numbers.len() {
        *current += 1;
        decrease(numbers, current, limit);
    } else {
        numbers[index as usize] -= 1;
    }
}

fn find_last(numbers: &Vec<usize>, n: usize, limit: usize) -> usize {
    for i in (0..=limit).rev() {
        if numbers[i] == n {
            return i;
        }
    }
    numbers.len()
}

fn find_first(numbers: &Vec<usize>, n: usize, limit: usize) -> usize {
    for i in 0..=limit {
        if numbers[i] == n {
            return i;
        }
    }
    numbers.len()
}

fn get_sum(numbers: &Vec<usize>) -> usize {
    let mut sum = 0;
    for i in 0..numbers.len() {
        let n = numbers[i];
        if n == 1 {
            return sum + numbers.len() - i;
        }
        sum += n;
    }
    sum
}

fn get_product(numbers: &Vec<usize>) -> usize {
    let mut product = 1;
    for &n in numbers {
        if n == 1 {
            return product;
        }
        product *= n;
    }
    product
}