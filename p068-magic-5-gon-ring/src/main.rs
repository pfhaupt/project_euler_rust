// https://projecteuler.net/problem=68

use itertools::Itertools;

#[derive(Debug, Clone)]
struct Segment {
    vals: Vec<u64>
}

impl Segment {
    fn set_element(&mut self, slot: usize, val: u64) {
        self.vals[slot] = val;
    }

    fn create() -> Segment {
        Segment { vals: vec![0, 0, 0] }
    }

    fn is_valid(&self, sum: u64) -> bool {
        self.vals.iter().sum::<u64>() == sum
    }

    fn equals(&self, other: &Segment) -> bool {
        for i in 0..self.vals.len() {
            if self.vals[i] != other.vals[i] {
                return false;
            }
        }
        true
    }
}

#[derive(Debug, Clone)]
struct Polygon {
    point_count: usize,
    segments: Vec<Segment>
}

impl Polygon {
    fn set_point(&mut self, slot: usize, val: u64) {
        if slot < self.segments.len() {
            self.segments[slot].set_element(0, val);
        } else {
            let line = slot % self.segments.len();
            self.segments[line].set_element(1, val);
            let line = if line as i64 - 1 < 0 { line + self.segments.len() - 1 } else { line - 1 };
            self.segments[line].set_element(2, val);
        }
    }

    fn add_segment(&mut self) {
        self.segments.push(Segment::create())
    }

    fn create(sides: usize) -> Polygon {
        let mut r = Polygon { point_count: 2 * sides, segments: vec![] };
        for _ in 0..sides {
            r.add_segment();
        }
        r
    }

    fn is_valid(&self, sum: u64) -> bool {
        self.segments.iter().all(|segment| segment.is_valid(sum))
    }

    fn get_point_count(&self) -> usize {
        self.point_count
    }

    #[allow(dead_code)]
    fn get_line_count(&self) -> usize {
        self.segments.len()
    }

    fn generate_string(&self) -> String {
        let mut result = String::from("");
        for segment in &self.segments {
            for val in &segment.vals {
                result += &val.to_string();
            }
        }
        result
    }
}

fn main() {
    /* 
    Very flexible (but bruteforce-y) implementation of the problem.
    I can generate any sized polygons, and specify any goal length.
    The solution itself is bruteforce:
    The polygon keeps track of how many points it has, I convert that to an array of [0, 1, .., size - 1]
    and iterate over all possible permutations of that array, setting each point to all values, effectively.
    With `cargo run --release` it finishes in ~7 seconds.

    Creating structs and all that was probably overkill, but I like my OOP. It was also a nice refresher for structs in Rust.
     */
    const SIZE: usize = 5;
    const TARGET_LEN: usize = 16;
    let mut p = Polygon::create(SIZE);
    let mut result = 0;
    // There's no basis for the 10..30 range, it just feels right
    for sum in 10..30 {
        let s = get_solutions(&mut p, sum);
        for poly in &s {
            let poly_as_string = poly.generate_string();
            if poly_as_string.len() == TARGET_LEN {
                let n: u64 = poly_as_string.parse().unwrap();
                if n > result {
                    result = n;
                }
            }
        }
    }
    println!("{}", result);
}

#[allow(dead_code)]
fn count_unique_solutions(polygon: &mut Polygon, goal: u64) -> u64 {
    let mut result = 0;
    let size = polygon.get_point_count();
    let mut permutations = (0..size).into_iter().permutations(size);
    while let Some(perm) = permutations.next() {
        for i in 0..size {
            polygon.set_point(i, perm[i] as u64 + 1);
        }
        if polygon.is_valid(goal) {
            result += 1;
        }
    }
    result / polygon.get_line_count() as u64
}

fn get_solutions(polygon: &mut Polygon, goal: u64) -> Vec<Polygon> {
    let mut result = vec![];
    let size = polygon.get_point_count();
    let mut permutations = (0..size).into_iter().permutations(size);
    while let Some(perm) = permutations.next() {
        for i in 0..size {
            polygon.set_point(i, perm[i] as u64 + 1);
        }
        if polygon.is_valid(goal) {
            result.push(polygon.clone());
        }
    }
    if result.len() == 0 {
        return vec![];
    }
    /* 
    Because of symmetry (just rotate the polygon a bit), each solution appears [line count] times in my result vector.
    I filter all of those repetitions, to only get unique solutions.
     */
    let mut unique: Vec<Polygon> = vec![result[0].clone()];
    for poly_1 in &result {
        let mut any_match = false;
        'poly_2: for poly_2 in &unique {
            let s_1 = &poly_1.segments;
            let s_2 = &poly_2.segments;
            for segment_1 in s_1 {
                for segment_2 in s_2 {
                    if segment_1.equals(&segment_2) {
                        any_match = true;
                        break 'poly_2;
                    }
                }
            }
        }
        if !any_match {
            unique.push(poly_1.clone());
        }
    }
    unique
}