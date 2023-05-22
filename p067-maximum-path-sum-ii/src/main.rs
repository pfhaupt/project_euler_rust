use std::fs;

fn main() {
    // https://projecteuler.net/problem=67
    let input = fs::read_to_string("src/triangle.txt")
    .expect("Something went wrong when loading the file!");

    let mut number_vec: Vec<Vec<u64>> = vec![];
    let lines = input.split("\n");
    for line in lines {
        let numbers = line.split(" ");
        let digits: Vec<u64> = numbers.map(|n| str::parse(n).unwrap()).collect();
        number_vec.push(digits);
    }

    const LEN: usize = 100;
    let mut arr: [[u64; LEN]; LEN] = [[0; LEN]; LEN];
    for i in 0..number_vec.len() {
        let row = &number_vec[i];
        for j in 0..row.len() {
            arr[i][j] = row[j];
        }
    }
    
    let mut new_arr: [[u64; LEN]; LEN] = [[0; LEN]; LEN];
    for i in 0..LEN {
        for j in 0..LEN {
            let row = j + i;
            new_arr[i][j] = if row < LEN { arr[row][j] } else { 0 };
        }
    }

    arr = [[0; LEN]; LEN];
    for x in 0..LEN {
        for y in 0..LEN {
            let mut max = 0;
            if x > 0 {
                max = u64::max(max, arr[x - 1][y]);
            }
            if y > 0 {
                max = u64::max(max, arr[x][y - 1]);
            }
            arr[x][y] = new_arr[x][y] + max;
        }
    }

    println!("{}", arr[LEN - 1][LEN - 1]);
}
