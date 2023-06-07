// https://projecteuler.net/problem=79

use std::fs;
use std::time::Instant;


#[derive(PartialEq, Clone)]
struct Order {
    a: u64,
    b: u64
}

fn get_digits(code: &u64) -> (u64, u64, u64) {
    let a = (code / 100) % 10;
    let b = (code / 10) % 10;
    let c = code % 10;
    (a, b, c)
}

fn get_orders(a: u64, b: u64, c: u64) -> Vec<Order> {
    vec![Order {a, b}, Order {a: b, b: c}, Order {a: a, b: c}]
}

fn get_indices(code: &Vec<u64>, order: &Order) -> (usize, usize) {
    let mut i_1 = 0;
    let mut i_2 = 0;
    for index in 0..code.len() {
        let digit = code[index];
        if digit == order.a {
            i_1 = index;
        }
        if digit == order.b {
            i_2 = index;
        }
    }
    (i_1, i_2)
}

fn sort_code(code: &mut Vec<u64>, orders: &Vec<Order>) {
    for order in orders {
        let (i_1, i_2) = get_indices(&code, order);
        if i_1 > i_2 {
            // Example: Passcode 180 says that there should be a 1 before an 8, but i_1 is bigger than i_2, which means 1 came after
            (code[i_1], code[i_2]) = (code[i_2], code[i_1]);
        }
    }
}

fn main() {
    /* 
    I could solve this by hand...
    319 -> 3 < 1 < 9
    680 -> 6 < 8 < 0
    180 -> 1 < 8 < 0 -> 6/1 < 8 < 0
    690 -> 6 < 9 < 0 -> 6/1 < 8/9 < 0
    129 -> 1 < 2 < 9 -> 6/1 < 2 < 8/9 < 0
    620 -> 6 < 2 < 0 -> 1/6 < 2 < 8/9 < 0
    762 -> 7 < 6 < 2 -> 7 < 1/6 < 2 < 8/9 < 0
    689 -> 6 < 8 < 9 -> 7 < 1/6 < 2 < 8 < 9 < 0
    762 -> 7 < 6 < 2 ->
    318 -> 3 < 1 < 8 -> 7/3 < 1/6 < 2 < 8 < 9 < 0
    368 -> 3 < 6 < 8 -> 7/3 < 1/6 < 2 < 8 < 9 < 0
    710 -> 7 < 1 < 0 -> 7/3 < 1/6 < 2 < 8 < 9 < 0
    720 -> 7 < 2 < 0 ->
    710 -> 7 < 1 < 0 ->
    629 -> 6 < 2 < 9 ->
    168 -> 1 < 6 < 8 -> 7/3 < 1 < 6 < 2 < 8 < 9 < 0
    160 ->
    689 ->
    716 -> 7 < 1 < 6 -> 7/3 < 1 < 6 < 2 < 8 < 9 < 0
    731 -> 7 < 3 < 1 -> 7 < 3 < 1 < 6 < 2 < 8 < 9 < 0 <- no 4s and 5s appear in the codes, so this is the solution
     */
    // This is kinda lazy - I'll come up with a nice solution later.
    // println!("73162890"); // :)

    /* 
    Actual solution:
    Keep track of a list of numbers, Keep track of every order
    At each step (each new passcode), insert any numbers into the list if they're not in there already
    Sort the list based on all orders
     */
    let now = Instant::now();
    let contents = fs::read_to_string("C:/Users/Philippe/RustProjects/project_euler_rust/p079-passcode-derivation/src/keylog.txt")
        .expect("Something went wrong when loading the file!");
    let passcodes: Vec<u64> = contents.split("\r\n").map(|s|s.parse().unwrap()).collect();
    let mut code_guess = vec![];
    let mut known_orders = vec![];
    for code in passcodes {
        let (a, b, c) = get_digits(&code);
        if !code_guess.contains(&a) {
            code_guess.push(a);
        }
        if !code_guess.contains(&b) {
            code_guess.push(b);
        }
        if !code_guess.contains(&c) {
            code_guess.push(c);
        }
        let orders = get_orders(a, b, c);
        for order in orders {
            if !known_orders.contains(&order) {
                known_orders.push(order);
            }
        }
        sort_code(&mut code_guess, &known_orders);
    }
    let result = code_guess.iter().map(|d| d.to_string()).collect::<Vec<String>>().join("");
    println!("{}", result);
    println!("{:?}", now.elapsed());
}
