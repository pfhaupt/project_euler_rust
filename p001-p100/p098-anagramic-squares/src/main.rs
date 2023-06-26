// https://projecteuler.net/problem=98

use std::time::Instant;
use std::fs;

use itertools::Itertools;

#[derive(Debug)]
struct Word {
    name: String,
    anagrams: Vec<Word>
}

impl Word {
    fn new(name: &String) -> Self {
        Word { name: name.clone(), anagrams: vec![] }
    }

    fn add(self: &mut Self, anagram: Word) {
        self.anagrams.push(anagram);
    }

    fn has_anagram(self: &Self, word: &Word) -> bool {
        let mut letter_count: [usize; 26] = [0; 26];
        for ch in self.name.chars() {
            letter_count[ch as usize - 65] += 1;
        }
        for ch in word.name.chars() {
            if letter_count[ch as usize - 65] == 0 {
                return false;
            }
            letter_count[ch as usize - 65] -= 1;
        }
        letter_count.iter().all(|c| *c == 0)
    }

    fn has_palindrome(self: &Self, word: &Word) -> bool {
        if self.name.len() != word.name.len() {
            false
        } else {
            self.name == word.name.chars().rev().collect::<String>()
        }
    }

    fn anagram_count(self: &Self) -> usize {
        self.anagrams.len()
    }
}

fn main() {
    let now = Instant::now();
    let contents = fs::read_to_string("src/words.txt")
        .expect("Something went wrong when loading the file!");

    let names: Vec<&str> = contents.split(",").collect();

    let mut words: Vec<String> = vec![];
    for name in names {
        words.push(name[1..(name.len()-1)].to_string());
    }

    let mut candidates = vec![];
    for w_1 in 0..words.len() {
        let mut w1 = Word::new(&words[w_1]);
        for w_2 in (w_1+1)..words.len() {
            let w2 = Word::new(&words[w_2]);
            if w1.has_anagram(&w2) {
                if !w1.has_palindrome(&w2) {
                    w1.add(w2);
                }
            }
        }
        if w1.anagram_count() > 0 {
            candidates.push(w1);
        }
    }

    // Candidates now only contains words where at least one anagram exists in the file

    for word in &candidates {
        // TODO: The actual task
        println!("{:?}", word);
    }
    println!("{:?}", now.elapsed());
}
