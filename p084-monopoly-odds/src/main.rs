// https://projecteuler.net/problem=84

use rand::{Rng, seq::SliceRandom};

struct Deck {
    index: usize,
    cards: Vec<usize>,
    cap: usize
}

impl Deck {
    fn get_next(&mut self) -> usize {
        let card = self.cards[self.index];
        self.index = (self.index + 1) % self.cap;
        card
    }
}
const MOVE_COUNT: usize = 10_000_000;
const DICE_SIDES: usize = 4;

const BOARD_SIZE: usize = 40;

const GO_TILE: usize = 0;
const JAIL_TILE: usize = 10;
const C1_TILE: usize = 11;
const E3_TILE: usize = 24;
const G2J_TILE: usize = 30;
const H2_TILE: usize = BOARD_SIZE - 1;
const R1_TILE: usize = 5;
const NEXT_R: usize = BOARD_SIZE * 2;
const NEXT_U: usize = BOARD_SIZE * 3;
const GO_BACK: usize = BOARD_SIZE * 4;
const NO_CARD: usize = 999999;

fn main() {
    let mut rng = rand::thread_rng();

    let mut cc_cards: Vec<usize> = vec![JAIL_TILE, GO_TILE];
    while cc_cards.len() != 16 { cc_cards.push(NO_CARD); }
    cc_cards.shuffle(&mut rng);
    let mut cc_deck = Deck { index: 0, cap: cc_cards.len(), cards: cc_cards };

    let mut ch_cards: Vec<usize> = vec![JAIL_TILE, GO_TILE, C1_TILE, E3_TILE, H2_TILE, R1_TILE, NEXT_R, NEXT_R, NEXT_U, GO_BACK];
    while ch_cards.len() != 16 { ch_cards.push(NO_CARD); }
    ch_cards.shuffle(&mut rng);
    let mut ch_deck = Deck { index: 0, cap: ch_cards.len(), cards: ch_cards };
    
    let mut count = vec![0; BOARD_SIZE];
    let mut position = 0;
    let mut double_streak = 0;
    for _ in 0..MOVE_COUNT {
        let first_dice = rng.gen_range(1..=DICE_SIDES);
        let second_dice = rng.gen_range(1..=DICE_SIDES);
        let total = first_dice + second_dice;
        if first_dice == second_dice {
            double_streak += 1;
        } else {
            double_streak = 0;
        }
        position = (position + total) % BOARD_SIZE;
        if double_streak == 3 {
            position = JAIL_TILE;
            double_streak = 0;
        }
        match position {
            G2J_TILE | JAIL_TILE => {
                position = JAIL_TILE;
            },
            2 | 17 | 33 => {
                let cc_card = cc_deck.get_next();
                if cc_card != NO_CARD {
                    position = cc_card;
                }
            },
            7 | 22 | 36 => {
                let ch_card = ch_deck.get_next();
                if ch_card != NO_CARD {
                    if ch_card != GO_TILE && ch_card % BOARD_SIZE == 0 {
                        if ch_card == GO_BACK {
                            position -= 3;
                            if position == 2 || position == 17 || position == 33 {
                                let cc_card = cc_deck.get_next();
                                if cc_card != NO_CARD {
                                    position = cc_card;
                                }
                            }
                        } else if ch_card == NEXT_R {
                            // railways are at 10*n + 5
                            while position % 10 != 5 {
                                position = (position + 1) % BOARD_SIZE;
                            }
                        } else if ch_card == NEXT_U {
                            // utility are at tile 12 and 28
                            if position > 28 || position <= 12 {
                                position = 12;
                            } else {
                                position = 28;
                            }
                        }
                    } else {
                        position = ch_card;
                    }
                }
            }
            _ => ()
        }
        count[position] += 1;
    }
    let mut indices: Vec<usize> = (0..count.len()).collect();
    indices.sort_by_key(|&i| &count[i]);
    indices.reverse();
    count.sort();
    let result = format!("{:02}{:02}{:02}", indices[0], indices[1], indices[2]);
    println!("{}", result);
}
