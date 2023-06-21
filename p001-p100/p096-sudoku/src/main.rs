// https://projecteuler.net/problem=96

use std::fmt::{Debug, Display, Formatter};
use std::fs;

const GRID_SIZE: usize = 9;
const ALL_ONES: usize = (2 << GRID_SIZE) - 2;

struct Sudoku {
    grid: [[usize; GRID_SIZE]; GRID_SIZE]
}

impl Sudoku {
    fn new(file: &String) -> Self {
        let chars: Vec<usize> = file.chars().filter(|c| *c != '\r' && *c != '\n').map(|c| (c as usize) - 48).collect();
        let mut current_index = 0;
        let mut grid = [[ALL_ONES; GRID_SIZE]; GRID_SIZE];
        for x in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                let digit = chars[current_index];
                if digit > 0 {
                    grid[x][y] = 1 << digit;
                }
                current_index += 1;
            }
        }
        Sudoku { grid }
    }

    fn get_grid_1d(&self) -> [usize; GRID_SIZE * GRID_SIZE] {
        let mut acc = [0; GRID_SIZE * GRID_SIZE];
        let mut index = 0;
        for row in self.grid {
            for elem in row {
                let mut digit = 0;
                if elem.count_ones() == 1 {
                    let mut number = elem;
                    loop {
                        if number & 1 == 1 {
                            break;
                        }
                        digit += 1;
                        number >>= 1;
                    };
                }
                acc[index] = digit;
                index += 1;
            }
        }
        acc
    }

    fn solve(&mut self) -> bool {
        let mut last_entropy = 0;
        'outer: loop {
            if !self.reduce_possible_candidates() {
                // Invalid sudoku!
                break false;
            }
            if let Some(entropy) = self.entropy() {
                if entropy == 0 {
                    break true;
                } else if entropy == last_entropy {
                    let before = self.grid.clone();
                    let candidate_index = self.get_tile_lowest_entropy();
                    let mut candidates = self.grid[candidate_index.0][candidate_index.1];
                    let mut bit_shift = 0;
                    while candidates & 1 == 0 {
                        bit_shift += 1;
                        candidates >>= 1;
                    }
                    let candidate = 1 << bit_shift;
                    self.grid[candidate_index.0][candidate_index.1] = candidate;
                    if self.solve() {
                        break 'outer true;
                    } else {
                        self.grid = before;
                        self.grid[candidate_index.0][candidate_index.1] = (candidates << bit_shift) ^ candidate;
                    }
                }
                last_entropy = entropy;
            } else {
                break false;
            }
        }
    }

    fn reduce_possible_candidates(&mut self) -> bool {
        for x in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                if self.grid[x][y] == 0 {
                    // No possible candidate -> Invalid sudoku configuration
                    return false;
                }
                let candidate_count = self.grid[x][y].count_ones();
                if candidate_count != 1 {
                    let mut next = usize::MAX.abs_diff(ALL_ONES);
                    let box_x = x / 3;
                    let box_y = y / 3;
                    for b_x_i in 0..3 {
                        for b_y_i in 0..3 {
                            let box_elem = self.grid[3 * box_x + b_x_i][3 * box_y + b_y_i];
                            if box_elem.count_ones() == 1 {
                                next |= box_elem;
                            }
                        }
                    }

                    for row_col in 0..GRID_SIZE {
                        let row_elem = self.grid[x][row_col];
                        let col_elem = self.grid[row_col][y];
                        if row_elem.count_ones() == 1 {
                            next |= row_elem;
                        }
                        if col_elem.count_ones() == 1 {
                            next |= col_elem;
                        }
                        // println!("{:b} {} {} {}", next, candidate_count, row_elem, col_elem);
                    }
                    // println!("{:010b}", !next);
                    self.grid[x][y] = !next;
                }
            }
        }
        true
    }

    fn get_tile_lowest_entropy(&self) -> (usize, usize) {
        let mut x = 0;
        let mut y = 0;
        let mut record_count = u32::MAX;
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                let count = self.grid[i][j].count_ones();
                if count == 1 { continue; }
                if count < record_count {
                    record_count = count;
                    x = i;
                    y = j;
                }
            }
        }
        (x, y)
    }

    fn entropy(&self) -> Option<u32> {
        let mut result = 0;
        for row in self.grid {
            for elem in row {
                result += elem.count_ones();
            }
        }
        if result >= 81 {
            Some(result - 81)
        } else {
            None
        }
    }
}

impl Debug for Sudoku {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut grid_as_str = String::new();
        for row in self.grid {
            for col in row {
                let count = col.count_ones();
                grid_as_str += &count.to_string();
            }
            grid_as_str += "\n";
        }
        write!(f, "{}", grid_as_str)
    }
}

impl Display for Sudoku {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut grid_as_str = String::new();
        for row in self.grid {
            for col in row {
                let mut index = 0;
                if col.count_ones() == 1 {
                    let mut number = col;
                    loop {
                        if number & 1 == 1 {
                            break;
                        }
                        index += 1;
                        number >>= 1;
                    };
                }
                grid_as_str += &index.to_string();
            }
            grid_as_str += "\n";
        }
        write!(f, "{}", grid_as_str)
    }
}

use std::time::Instant;

fn main() {
    let now = Instant::now();
    let contents = fs::read_to_string("src/sudoku.txt")
        .expect("Something went wrong when loading the file!");
    let grid_lines: Vec<&str> = contents.lines().filter(|l| !l.starts_with("Grid")).collect();

    let mut result = 0;

    let mut index = 0;
    for _ in 0..50 {
        let mut sudoku = String::new();
        for _ in 0..9 {
            sudoku += grid_lines[index];
            index += 1;
        }
        let mut sudoku = Sudoku::new(&sudoku);
        sudoku.solve();
        let first_digits = &sudoku.get_grid_1d()[0..3];
        let mut result_string = String::new();
        for digit in first_digits {
            result_string += &digit.to_string();
        }
        match result_string.parse::<usize>() {
            Ok(sum) => result += sum,
            Err(e) => panic!("{}", e)
        }
    }
    println!("{}", result);
    println!("{:?}", now.elapsed());
}
