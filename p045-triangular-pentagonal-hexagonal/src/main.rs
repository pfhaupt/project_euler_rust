// https://projecteuler.net/problem=45

fn main() {
    let mut tri_index: u64 = 1;
    let mut pen_index = 1;
    let mut hex_index = 2;
    let mut tri_number = 1;
    let mut hex_number = 1;
    let mut pen_number = 1;
    loop {
        if tri_number > 40755 && tri_number == pen_number && hex_number == pen_number {
            break;
        }
        hex_number = hex_index * (2 * hex_index - 1);
        hex_index += 1;
        while pen_number < hex_number {
            pen_number = pen_index * (3 * pen_index - 1) / 2;
            pen_index += 1;
        }
        while tri_number < pen_number {
            tri_number = tri_index * (tri_index + 1) / 2;
            tri_index += 1;
        }
    }
    println!("{}", tri_number);
}
