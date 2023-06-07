// https://projecteuler.net/problem=54

use std::fs;
use std::fmt::{Debug, Formatter};
use std::time::Instant;


#[derive(Clone)]
struct Card {
    suit: u8,
    value: u8
}

impl Debug for Card {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}{}", &VALUES[self.value as usize], &SUITS[self.suit as usize])
     }
}

struct Hand {
    cards: Vec<Card>
}

const VALUES: [&str; 13] = ["2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A"];
const SUITS: [&str; 4] = ["H", "C", "S", "D"];

const TEN: u8 = 84;
const JACK: u8 = 74;
const QUEEN: u8 = 81;
const KING: u8 = 75;
const ACE: u8 = 65;

const HEART: u8 = 72;
const CLUB: u8 = 67;
const SPADE: u8 = 83;
const DIAMOND: u8 = 68;

const HIGH_CARD: u8 = 0;
const ONE_PAIR: u8 = 1;
const TWO_PAIRS: u8 = 2;
const THREE_OF_A_KIND: u8 = 3;
const STRAIGHT: u8 = 4;
const FLUSH: u8 = 5;
const FULL_HOUSE: u8 = 6;
const FOUR_OF_A_KIND: u8 = 7;
const STRAIGHT_FLUSH: u8 = 8;
const ROYAL_FLUSH: u8 = 9;

fn main() {
    let now = Instant::now();
    let contents = fs::read_to_string("C:/Users/Philippe/RustProjects/project_euler_rust/p054-poker-hands/src/poker.txt")
        .expect("Something went wrong when loading the file!");
    let hands: Vec<&str> = contents.split("\r\n").collect();
    let mut rounds: Vec<Vec<&str>> = vec![];
    for hand in hands {
        let round = hand.split(" ").collect();
        rounds.push(round);
    }
    let mut player_one_wins = 0;
    for round in rounds {
        let mut player_one = get_hand(&round, 0);
        let mut player_two = get_hand(&round, 1);
        player_one.cards.sort_by(|c1, c2|c1.value.cmp(&c2.value));
        player_two.cards.sort_by(|c1, c2|c1.value.cmp(&c2.value));
        let rank_one = get_ranking(&player_one);
        let rank_two = get_ranking(&player_two);
        if rank_one.0 > rank_two.0 {
            // println!("Clear Win for player 1:       {:?} {:?}", player_one.cards, player_two.cards);
            player_one_wins += 1;
        } else if rank_one.0 < rank_two.0 {
            // println!("Clear Win for player 2:       {:?} {:?}", player_one.cards, player_two.cards);
        } else {
            let winner = evaluate(&player_one, &player_two);
            // println!("Same rank, but player {} wins: {:?} {:?}", winner, player_one.cards, player_two.cards);
            if winner == 1 {
                player_one_wins += 1;
            }
        }
    }
    println!("{}", player_one_wins);
    println!("{:?}", now.elapsed());
}

fn evaluate(player_one: &Hand, player_two: &Hand) -> u8 {
    let rank_one = get_ranking(player_one);
    let rank_two = get_ranking(player_two);
    let rank = rank_one.0;
    assert_eq!(rank_two.0, rank);
    match rank {
        HIGH_CARD => return evaluate_high_card(player_one, player_two),
        ONE_PAIR => {
            let v1 = player_one.cards[rank_one.1[0]].value;
            let v2 = player_two.cards[rank_two.1[0]].value;
            if v1 > v2 {
                return 1;
            } else if v2 > v1 {
                return 2;
            } else {
                return evaluate_high_card(player_one, player_two);
            }
        }
        _ => todo!("Handle {}", rank)
    }
}

fn evaluate_high_card(player_one: &Hand, player_two: &Hand) -> u8 {
    // Because hands are sorted, we can just start at the end and compare each value
    for index in (0..=4).rev() {
        let v1 = player_one.cards[index].value;
        let v2 = player_two.cards[index].value;
        if v1 > v2 {
            return 1;
        } else if v2 > v1 {
            return 2;
        }
    }
    panic!("Given the task, this situation in evaluate_high_card() is unreachable.");
}

fn get_ranking(hand: &Hand) -> (u8, Vec<usize>) {
    let all_indexes = vec![0, 1, 2, 3, 4];
    let royal_flush = is_royal_flush(hand);
    if royal_flush {
        return (ROYAL_FLUSH, all_indexes);
    }
    let straight_flush = is_straight_flush(hand);
    if straight_flush {
        return (STRAIGHT_FLUSH, all_indexes);
    }
    let fours = is_four_of_a_kind(hand);
    if fours.0 {
        return (FOUR_OF_A_KIND, fours.1);
    }
    let full_house = is_full_house(hand);
    if full_house.0 {
        return (FULL_HOUSE, full_house.1);
    }
    let flush = is_flush(hand);
    if flush {
        return (FLUSH, all_indexes);
    }
    let straight = is_straight(hand);
    if straight {
        return (STRAIGHT, all_indexes);
    }
    let threes = is_three_of_a_kind(hand);
    if threes.0 {
        return (THREE_OF_A_KIND, threes.1);
    }
    let twos = is_two_pairs(hand);
    if twos.0 {
        return (TWO_PAIRS, twos.1);
    }
    let pair = is_one_pair(hand);
    if pair.0 {
        return (ONE_PAIR, pair.1);
    }
    (HIGH_CARD, vec![])
}

fn is_royal_flush(hand: &Hand) -> bool {
    if !is_flush(hand) {
        return false;
    }
    const NEEDED: [u8; 5] = [TEN, JACK, QUEEN, KING, ACE];
    for i in 0..5 {
        if hand.cards[i].value != NEEDED[i] {
            return false;
        }
    }
    true
}
fn is_straight_flush(hand: &Hand) -> bool {
    is_straight(hand) && is_flush(hand)
}
fn is_four_of_a_kind(hand: &Hand) -> (bool, Vec<usize>) {
    for i in 0..=1 {
        let value = hand.cards[i].value;
        let mut four = true;
        for j in 0..4 {
            if hand.cards[i + j].value != value {
                four = false;
                break;
            }
        }
        if four {
            return (true, vec![i, i + 1, i + 2, i + 3]);
        }
    }
    (false, vec![])
}

fn is_full_house(hand: &Hand) -> (bool, Vec<usize>) {
    let v1 = hand.cards[0].value;
    let v2 = hand.cards[1].value;
    let v3 = hand.cards[2].value;
    let v4 = hand.cards[3].value;
    let v5 = hand.cards[4].value;
    if v1 != v2 {
        return (false, vec![]);
    } else if v4 != v5 {
        return (false, vec![]);
    } else if v3 == v2 {
        // v1, v2, v3 is triple, v4, v5 is pair
        return (true, vec![0, 3]);
    } else if v3 == v4 {
        // v1, v2 is pair, v3, v4, v5 is triple
        return (true, vec![0, 2]);
    } else {
        // v1 == v2, v4 == v5, but v3 != v2 != v4
        // => Two pairs, but not a full house.
        return (false, vec![]);
    }
}

fn is_flush(hand: &Hand) -> bool {
    let suit = hand.cards[0].suit;
    hand.cards.iter().all(|c|c.suit == suit)
}
fn is_straight(hand: &Hand) -> bool {
    let mut last = hand.cards[0].value;
    for i in 1..5 {
        let value = hand.cards[i].value;
        if value - last != 1 {
            return false;
        }
        last = value;
    }
    true
}
fn is_three_of_a_kind(hand: &Hand) -> (bool, Vec<usize>) {
    for i in 0..=2 {
        let value = hand.cards[i].value;
        let mut three = true;
        for j in 0..3 {
            if hand.cards[i + j].value != value {
                three = false;
                break;
            }
        }
        if three {
            return (true, vec![i, i + 1, i + 2]);
        }
    }
    (false, vec![])
}
fn is_two_pairs(hand: &Hand) -> (bool, Vec<usize>) {
    let v1 = hand.cards[0].value;
    let v2 = hand.cards[1].value;
    let v3 = hand.cards[2].value;
    let v4 = hand.cards[3].value;
    let v5 = hand.cards[4].value;
    if v1 != v2 {
        // because hand is sorted, the only way of having two pairs is v2==v3 and v4==5 if v1!=v2
        if v2 != v3 || v4 != v5 {
            return (false, vec![]);
        } else {
            // v2 == v3, v4 == v5, we have two pairs.
            // Technically this could still mean a four of a kind, but that's checked before two_pairs.
            return (true, vec![1, 3]);
        }
    } else {
        // v1 and v2 are a pair
        if v3 != v4 {
            if v4 != v5 {
                // v3 != v4 != v5, normal pair
                return (false, vec![]);
            } else {
                return (true, vec![0, 3]);
            }
        } else {
            // v1, v2 and v3, v4 are a pair
            if v4 == v5 {
                // Full house!
                return (false, vec![]);
            } else {
                return (true, vec![0, 2]);
            }
        }
    }
}

fn is_one_pair(hand: &Hand) -> (bool, Vec<usize>) {
    for i in 0..=3 {
        let value = hand.cards[i].value;
        let mut two = true;
        for j in 0..2 {
            if hand.cards[i + j].value != value {
                two = false;
                break;
            }
        }
        if two {
            return (true, vec![i, i + 1]);
        }
    }
    (false, vec![])
}

fn get_hand(round: &Vec<&str>, player: usize) -> Hand {
    let start = 0 + (player * 5);
    let mut cards = vec![];
    for i in 0..5 {
        let index = start + i;
        let card = get_card(round[index]);
        cards.push(card);
    }
    Hand { cards }
}

fn get_card(input: &str) -> Card {
    let input = input.as_bytes();
    let value = get_value(input[0]);
    let suit = get_suit(input[1]);
    Card { suit, value }
}

fn get_value(bla: u8) -> u8 {
    if bla >= 48 && bla <= 57 {
        return bla - 50;
    }
    if bla == TEN {
        return 8;
    } else if bla == JACK {
        return 9;
    } else if bla == QUEEN {
        return 10;
    } else if bla == KING {
        return 11;
    } else if bla == ACE {
        return 12;
    }
    panic!("Undefined value! {}", bla);
}

fn get_suit(bla: u8) -> u8 {
    if bla == HEART {
        return 0;
    } else if bla == CLUB {
        return 1;
    } else if bla == SPADE {
        return 2;
    } else if bla == DIAMOND {
        return 3;
    }
    panic!("Undefined suit! {}", bla);
}