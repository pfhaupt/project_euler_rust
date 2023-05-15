// The problem asks for a 20x20 grid, but I am calculating the paths for each vertex,
// and a 20x20 grid means 21 vertices per row/column.
const WIDTH: usize = 21;
const HEIGHT: usize = WIDTH;


fn main() {
    /*
    This solution implements a dynamic programming solution:
    Starting at the edge cases (every vertex at the bottom or right edge has exactly one possible path),
    I construct every other vertex path-count by following a single observation:
    The path-count for any cell is the amount of paths achieved by going down plus the paths achieved by going right
    Or, more formally:
    For any given cell (x, y), let f(x, y) be the amount of paths from this cell to the bottom right, then:
    f(x, y) = f(x + 1, y) + f(x, y + 1)
    f(x, 0) = 1
    f(0, y) = 1
    This recursive definition can be implemented by looping over a two-dimensional array.

    u128 allows up to WIDTH = HEIGHT = 66 (67 overflows), and the solution prints instantly. This is very impressive.
     */

    // Initialize the grid with 0's.
    let mut arr: [[u64; WIDTH]; HEIGHT] = [[0; WIDTH]; HEIGHT];

    for i in 0..WIDTH {
        // Any cell at the edge of the grid only has 1 possible path
        // This is the base case, every other position can be calculated recursively
        arr[i][HEIGHT - 1] = 1;
        arr[WIDTH - 1][i] = 1;
    }
    for x in (0..(WIDTH-1)).rev() {
        for y in (0..(HEIGHT-1)).rev() {
            // x is in range [WIDTH - 1, 0]
            // y is in range [HEIGHT - 1, 0]
            let mut cum: u64 = 0;
            // How many possible paths do we get by going down?
            cum += arr[x][y + 1];
            // How many possible paths do we get by going right?
            cum += arr[x + 1][y];
            arr[x][y] = cum;
            // We could just replace the lines above with this one-liner:
            // arr[x][y] = arr[x][y + 1] + arr[x + 1][y];
        }
    }

    for row in arr {
        for elem in row {
            print!("{:05} ", elem);
        }
        println!();
    }

    // The problem asked for getting from the top left to the bottom right, which is exactly what (0, 0) contains.
    println!("{}", arr[0][0]);
}