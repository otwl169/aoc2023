use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::zip;
use std::fs::read_to_string;

#[derive(Debug)]
struct Hand {
    cards: String,
    bet_amount: u32,
}

impl Hand {
    fn get_type(&self) -> u32 {
        let mut card_counts = HashMap::new();
        for c in self.cards.chars() {
            card_counts.entry(c).and_modify(|count| *count+=1).or_insert(1);
        }

        let mut five_of_a_kind = false;
        let mut four_of_a_kind = false;
        let mut three_of_a_kind = false;
        let mut pairs = 0;
        for (_k, &v) in card_counts.iter() {
            if v == 5 {
                five_of_a_kind = true;
            } else if v == 4 {
                four_of_a_kind = true;
            } else if v == 3 {
                three_of_a_kind = true;
            } else if v == 2 {
                pairs += 1;
            }
        }

        if five_of_a_kind {
            return 6;
        } else if four_of_a_kind {
            return 5;
        } else if three_of_a_kind && pairs == 1 {
            return 4;
        } else if three_of_a_kind {
            return 3;
        } else if pairs == 2 {
            return 2;
        } else if pairs == 1 {
            return 1;
        } else {
            return 0;
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let card_values: HashMap<char, u32> = HashMap::from([
            ('A', 14),
            ('K', 13),
            ('Q', 12),
            ('J', 11),
            ('T', 10),
            ('9', 9),
            ('8', 8),
            ('7', 7),
            ('6', 6),
            ('5', 5),
            ('4', 4),
            ('3', 3),
            ('2', 2),
        ]);

        if self.get_type() != other.get_type() {
            return self.get_type().cmp(&other.get_type());
        } else {
            let mut result = Ordering::Equal;
            let cards1 = self.cards.chars();
            let cards2 = other.cards.chars();
            for (c1, c2) in zip(cards1, cards2) {
                if result == Ordering::Equal && card_values[&c1] != card_values[&c2] {
                    result = card_values[&c1].cmp(&card_values[&c2]);
                }
            }
            return result;
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

pub fn day7_part1() {

    let mut games: Vec<Hand> = read_to_string("input/07.txt")
        .unwrap()
        .lines()
        .map(|x| {
            // let line = x.to_string();
            let (cards, bet_amount) = x.split_once(' ').unwrap();
            let hand: Hand = Hand {cards: cards.to_string(), bet_amount: bet_amount.parse().unwrap()};
            
            hand
        })
        .collect::<Vec<Hand>>();
    
    games.sort();
    
    
    let ans: u64 = games.iter()
        .enumerate()
        .map(|(i, hand)| {(i+1) as u64 * hand.bet_amount as u64})
        .sum();
    
    // println!("{:?}", games);
    // println!("{:?}", games.len());
    println!("{}", ans);
}

#[derive(Debug, Clone)]
struct Hand2 {
    cards: String,
    bet_amount: u32,
}

impl Hand2 {
    fn get_type(&self) -> u32 {
        let mut card_counts = HashMap::new();
        for c in self.cards.chars() {
            card_counts.entry(c).and_modify(|count| *count+=1).or_insert(1);
        }

        let mut five_of_a_kind = false;
        let mut four_of_a_kind = false;
        let mut three_of_a_kind = false;
        let mut pairs = 0;
        for (_k, &v) in card_counts.iter() {
            if v == 5 {
                five_of_a_kind = true;
            } else if v == 4 {
                four_of_a_kind = true;
            } else if v == 3 {
                three_of_a_kind = true;
            } else if v == 2 {
                pairs += 1;
            }
        }

        let jokers = *card_counts.get(&'J').unwrap_or(&0);
        if five_of_a_kind {
            return 6;
        } else if four_of_a_kind {
            return if jokers > 0 {6} else {5}; // with joker = five of a kind
        } else if three_of_a_kind && pairs == 1 {
            return if jokers > 0 {6} else {4}; // with jokers = five of a kind
        } else if three_of_a_kind {
            return if jokers > 0 {5} else {3}; // with joker can make a four of a kind
        } else if pairs == 2 {
            if jokers == 2 {
                return 5; // joker pair = 4 of a kind
            } else if jokers == 1 {
                return 4; // joker not pair = full house
            } else {
                return 2; // no joker = 2 pair
            }
        } else if pairs == 1 {
            return if jokers > 0 {3} else {1}; // 3 of a kind with joker or joker pairx
        } else {
            return if jokers > 0 {1} else {0};
        }
    }
}

impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> Ordering {
        let card_values: HashMap<char, u32> = HashMap::from([
            ('A', 14),
            ('K', 13),
            ('Q', 12),
            ('T', 10),
            ('9', 9),
            ('8', 8),
            ('7', 7),
            ('6', 6),
            ('5', 5),
            ('4', 4),
            ('3', 3),
            ('2', 2),
            ('J', 1),
        ]);

        if self.get_type() != other.get_type() {
            return self.get_type().cmp(&other.get_type());
        } else {
            let mut result = Ordering::Equal;
            let cards1 = self.cards.chars();
            let cards2 = other.cards.chars();
            for (c1, c2) in zip(cards1, cards2) {
                if result == Ordering::Equal && card_values[&c1] != card_values[&c2] {
                    result = card_values[&c1].cmp(&card_values[&c2]);
                }
            }
            return result;
        }
    }
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand2 {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand2 {}

pub fn day7_part2() {
    println!("Day 7 part 2");

    let mut games: Vec<Hand2> = read_to_string("input/07.txt")
        .unwrap()
        .lines()
        .map(|x| {
            // let line = x.to_string();
            let (cards, bet_amount) = x.split_once(' ').unwrap();
            let hand: Hand2 = Hand2 {cards: cards.to_string(), bet_amount: bet_amount.parse().unwrap()};
            
            hand
        })
        .collect::<Vec<Hand2>>();
    
    games.sort();
    
    
    let ans: u64 = games.iter()
        .enumerate()
        .map(|(i, hand)| {(i+1) as u64 * hand.bet_amount as u64})
        .sum();
    
    // let four_of_a_kinds: Vec<_> = games.iter().cloned()
    //     .filter(|x| x.get_type() == 0)
    //     .collect();
    
    // println!("{:?}", four_of_a_kinds);
    // println!("{:?}", games.len());
    println!("{}", ans);
}