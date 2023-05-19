use std::fs;
use std::collections::HashSet;

fn main() {
    // https://projecteuler.net/problem=59
    let contents = fs::read_to_string("src/cipher.txt").expect("Something went wrong when loading the file!");
    let bytes: Vec<u8> = contents.split(",").map(|s| s.parse().unwrap()).collect();
    let first_candidates: HashSet<u8> = get_key_part(&bytes, 0);
    let second_candidates: HashSet<u8> = get_key_part(&bytes, 1);
    let third_candidates: HashSet<u8> = get_key_part(&bytes, 2);
    let mut first_key = 0;
    let mut second_key = 0;
    let mut third_key = 0;
    const COMMON_WORDS: [&str; 5] = ["the", "is", "a", "of", "this"];
    'outer: for first_cand in &first_candidates {
        for second_cand in &second_candidates {
            for third_cand in &third_candidates {
                let mut result = String::from("");
                for i in (0..bytes.len()).step_by(3) {
                    let e1 = bytes[i] ^ first_cand;
                    let e2 = bytes[i + 1] ^ second_cand;
                    let e3 = bytes[i + 2] ^ third_cand;
                    result += &(e1 as char).to_string();
                    result += &(e2 as char).to_string();
                    result += &(e3 as char).to_string();
                }
                let valid_english = COMMON_WORDS.iter().all(|word|result.contains(word));
                if valid_english {
                    first_key = *first_cand;
                    second_key = *second_cand;
                    third_key = *third_cand;
                    break 'outer;
                }
            }
        }
    }
    let mut sum = 0u64;
    for i in (0..bytes.len()).step_by(3) {
        let e1 = bytes[i] ^ first_key;
        let e2 = bytes[i + 1] ^ second_key;
        let e3 = bytes[i + 2] ^ third_key;
        sum += e1 as u64;
        sum += e2 as u64;
        sum += e3 as u64;
    }
    println!("{}", sum);
}

fn get_key_part(bytes: &Vec<u8>, key_index: usize) -> HashSet<u8> {
    let mut result = HashSet::new();
    for k in 97..=122 {
        for i in (0..bytes.len()).step_by(3) {
            let v = is_valid(bytes[i + key_index], k);
            result.insert(k);
            if !v {
                result.remove(&k);
                break;
            }
        }
    }
    result
}

fn is_valid(val: u8, key: u8) -> bool {
    let e = val ^ key;
    if 97 <= e && e <= 122 {
        return true;
    } else if 32 <= e && e <= 97 {
        return true;
    }
    false
}