// https://projecteuler.net/problem=82

use std::fs;

fn main() {
    /* 
    Classic Dijkstra to traverse the "map" of "rough paths".
     */
    let input = fs::read_to_string("C:/Users/Philippe/RustProjects/project_euler_rust/p082-path-sum-three-ways/src/matrix.txt")
                    .expect("Something went wrong when loading the file!");
    let lines = input.lines();
    let mut matrix = vec![];
    for line in lines {
        let mut row: Vec<u64> = vec![];
        let values = line.split( ",");
        for value in values {
            row.push(value.parse().unwrap());
        }
        matrix.push(row);
    }
    let mut result = u64::MAX;
    for i in 0..matrix.len() {
        let slot = dijkstra(&matrix, (i, 0));
        if slot < result {
            result = slot;
        }
    }
    println!("{}", result);
}

fn dijkstra(matrix: &Vec<Vec<u64>>, start: (usize, usize)) -> u64 {
    let mut dist = vec![vec![u64::MAX; matrix.len()]; matrix.len()];
    let mut queue = vec![];
    for x in 0..matrix.len() {
        for y in 0..matrix.len() {
            queue.push((x, y));
        }
    }
    dist[start.0][start.1] = matrix[start.0][start.1];
    while !queue.is_empty() {
        let closest = get_closest(&queue, &dist);
        if closest.1.1 == matrix.len() - 1 {
            break;
        }
        queue.remove(closest.0);
        for neighbor in get_neighbors(&closest.1, matrix.len()) {
            let alt = dist[closest.1.0][closest.1.1] + (matrix[neighbor.0][neighbor.1]);
            if alt < dist[neighbor.0][neighbor.1] {
                dist[neighbor.0][neighbor.1] = alt;
            }
        }
    }
    let mut shortest = u64::MAX;
    for i in 0..matrix.len() {
        if dist[i][matrix.len() - 1] < shortest {
            shortest = dist[i][matrix.len() - 1];
        }
    }
    shortest
}

fn get_neighbors(point: &(usize, usize), board_size: usize) -> Vec<(usize, usize)> {
    let mut result = vec![];
    if point.1 + 1 < board_size {
        // Right
        result.push((point.0, point.1 + 1));
    }
    if point.0 + 1 < board_size {
        // Down
        result.push((point.0 + 1, point.1));
    }
    if point.0 > 0 {
        // Up
        result.push((point.0 - 1, point.1));
    }
    result
}

fn get_closest(queue: &Vec<(usize, usize)>, distances: &Vec<Vec<u64>>) -> (usize, (usize, usize)) {
    let mut index = 0;
    let mut shortest_distance = u64::MAX;
    for i in 0..queue.len() {
        let elem = queue[i];
        let dist = distances[elem.0][elem.1];
        if dist < shortest_distance {
            shortest_distance = dist;
            index = i;
        }
    }
    (index, queue[index])
}