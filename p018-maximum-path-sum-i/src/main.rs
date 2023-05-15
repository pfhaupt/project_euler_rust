fn main() {
    // https://projecteuler.net/problem=18
    let input = 
"75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23";
    let mut number_vec: Vec<Vec<u64>> = vec![];
    let lines = input.split("\n");
    for line in lines {
        let numbers = line.split(" ");
        let digits: Vec<u64> = numbers.map(|n| str::parse(n).unwrap()).collect();
        number_vec.push(digits);
    }
    // number_vec now contains the parsed numbers, in the same form as the input string

    const LEN: usize = 15; // last line has 15 numbers
    let mut arr: [[u64; LEN]; LEN] = [[0; LEN]; LEN];
    for i in 0..number_vec.len() {
        let row = &number_vec[i];
        for j in 0..row.len() {
            arr[i][j] = row[j];
        }
    }
    for row in arr {
        for elem in row {
            print!("{:02} ", elem);
        }
        println!();
    }
    println!();
    
    /* 
    arr now is a "triangle" of numbers, the top right half is zeros, the bottom left half contains the input 1:1
    For my idea, I need to "gravitate" the array, i.e:
    75 00 00
    95 64 00
    17 47 82
    turns into
    75 64 82
    95 47 00
    17 00 00
    such that now the values are in the top left, and the zeroes are in the bottom right
     */
    let mut new_arr: [[u64; LEN]; LEN] = [[0; LEN]; LEN];
    for i in 0..LEN {
        for j in 0..LEN {
            // with each column, it needs to shift one down
            let row = j + i;
            new_arr[i][j] = if row < LEN { arr[row][j] } else { 0 };
        }
    }

    for row in new_arr {
        for elem in row {
            print!("{:02} ", elem);
        }
        println!();
    }
    println!();
    
    /*
    new_arr is now structured in a way that I can apply some dynamic programming to it :D
    The idea is as follows: The problem asks for the maximum total sum from top to bottom
    For any given cell (x, y) in that array, let f(x, y) be the total sum of that cell from (0, 0) ("top of the triangle") to that cell.
    A cell in the triangle always has two possible sums: When coming from the top right, or top left cell.
    In the array, this cell is either to the left ("top right") or to the top ("top left").
    So I just check which one is higher, and add the current cell value to it.
    Or, more formally:
    c(x, y) = starting value of cell (x, y)
    f(x, y) = c(x, y) + max(f(x - 1, y), f(x, y - 1))
    f(x, 0) = c(x, 0) + f(x - 1, 0)
    f(0, y) = c(0, y) + f(0, y - 1)
    f(0, 0) = c(0, 0)
     */
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
    for row in arr {
        for elem in row {
            print!("{:04} ", elem);
        }
        println!();
    }
    println!();
    println!("{}", arr[LEN - 1][LEN - 1]);
}
