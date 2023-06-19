// https://projecteuler.net/problem=93

use std::time::Instant;
use itertools::Itertools;
use std::collections::BTreeSet;

// https://github.com/rekka/meval-rs
extern crate meval;

#[derive(Clone, Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div
}

fn op_as_str(op: &Operation) -> &str {
    match op {
        Operation::Add => "+",
        Operation::Sub => "-",
        Operation::Mul => "*",
        Operation::Div => "/"
    }
}

fn main() {
    /* 
    Very inefficient implementation, we generate all possible Strings for every tuple.
    Instead we could just generate all possible templates first, then use meval to replace a,b,c,d with the digits.
     */
    let now = Instant::now();
    let operations = vec![Operation::Add, Operation::Sub, Operation::Mul, Operation::Div];
    let combinations = operations.iter()
                                            .cartesian_product(&operations)
                                            .cartesian_product(&operations)
                                            .map(|t|vec![t.0.0, t.0.1, t.1])
                                            .collect_vec();
    let mut result = 0;
    let mut record = vec![];
    for d in 1..=9 {
        for c in 1..d {
            for b in 1..c {
                for a in 1..b {
                    let mut hits = BTreeSet::new();
                    let numbers: Vec<i16> = vec![a, b, c, d];
                    let mut number_perm = numbers.iter().permutations(numbers.len());
                    while let Some(perm) = number_perm.next() {
                        let perm = perm.iter().map(|v|**v).collect_vec();
                        let result = calculate(&perm, &combinations);
                        hits.extend(result);
                    }
                    let mut highest = 0;
                    for n in &hits {
                        if n - highest == 1 {
                            highest = *n;
                        } else {
                            break;
                        }
                    }
                    if highest > result {
                        result = highest;
                        record = numbers;
                    }
                }
            }
        }
    }
    println!("{}", record.iter().map(|v|v.to_string()).join(""));
    println!("{:?}", now.elapsed());
}

fn calculate(numbers: &Vec<i16>, operations: &Vec<Vec<&Operation>>) -> Vec<i16> {
    let mut result = vec![];
    for op in operations {
        let mut as_str = String::new();
        for i in 0..op.len() {
            as_str += &(numbers[i].to_string() + op_as_str(op[i]));
        }
        as_str += &numbers.last().unwrap().to_string();
        let with_paren = wrap_into_paren(&as_str);
        for equation in with_paren {
            if let Some(answer) = evaluate(&equation) {
                if answer > 0 && !result.contains(&answer) {
                    result.push(answer);
                }
            }
        }
    }
    result.sort();
    result
}

fn wrap_into_paren(equation: &String) -> Vec<String> {
    let mut result = vec![];
    for start_paren in (0..equation.len()).step_by(2) {
        let before_open_paren = String::from(&equation[..start_paren]);
        let after_open_paren = String::from(&equation[start_paren..]);
        let new_str = before_open_paren + "(" + &after_open_paren;
        for end_paren in (start_paren..equation.len()).step_by(2) {
            let end_paren = end_paren + 2;
            let before_end_paren = String::from(&new_str[..end_paren]);
            let after_end_paren = String::from(&new_str[end_paren..]);
            let new_str = before_end_paren + ")" + &after_end_paren;
            result.push(new_str);
        }
    }
    result
}

fn evaluate(equation: &String) -> Option<i16> {
    let r = meval::eval_str(equation);
    if r.is_ok() {
        let r = r.unwrap();
        if r.fract() == 0.0 {
            Some(r as i16)
        } else {
            None
        }
    } else {
        None
    }
}