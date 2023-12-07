use std::{collections::HashMap, ops::Deref};

use crate::util::Puzzles;

const CARDS: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];
const CARDS_JOKER: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

fn get_value(card: char, joker: bool) -> u32 {
    let array = if joker { CARDS_JOKER } else { CARDS };
    return array.iter().position(|c| *c == card).unwrap() as u32;
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
enum Kind {
    Five,
    Four,
    FullHouse,
    Three,
    TwoPair,
    Pair,
    High,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Hand([char; 5], Kind, bool);

impl Hand {
    fn parse_kind(text: &str, joker: bool) -> Kind {
        let cards = text.chars().collect::<Vec<_>>();
        let card_count = cards.iter().fold(HashMap::new(), |mut map, c| {
            map.insert(*c, map.get(c).unwrap_or(&0) + 1);
            map
        });
        match card_count
            .keys()
            .map(|k| card_count.get(k).unwrap())
            .max()
            .unwrap()
        {
            5 => return Kind::Five,
            4 => {
                if joker && card_count.contains_key(&'J') {
                    return Kind::Five;
                }

                return Kind::Four;
            }
            3 => {
                if joker
                    && card_count
                        .keys()
                        .max_by_key(|k| card_count.get(k).unwrap())
                        .unwrap()
                        == &'J'
                {
                    if card_count.keys().len() == 3 {
                        return Kind::Four;
                    } else {
                        return Kind::Five;
                    }
                }

                if joker && card_count.contains_key(&'J') {
                    if card_count.get(&'J').unwrap() == &1 {
                        return Kind::Four;
                    } else {
                        return Kind::Five;
                    }
                }

                if card_count
                    .keys()
                    .find(|k| card_count.get(&k).unwrap() == &2)
                    .is_some()
                {
                    return Kind::FullHouse;
                }

                return Kind::Three;
            }
            2 => {
                if card_count
                    .keys()
                    .filter(|k| card_count.get(&k).unwrap() == &2)
                    .count()
                    == 2
                {
                    if joker && card_count.get(&'J') == Some(&1) {
                        return Kind::FullHouse;
                    }

                    if joker && card_count.get(&'J') == Some(&2) {
                        return Kind::Four;
                    }

                    return Kind::TwoPair;
                }

                if joker && card_count.get(&'J') == Some(&1) {
                    return Kind::Three;
                }

                if joker && card_count.get(&'J') == Some(&2) {
                    return Kind::Three;
                }

                return Kind::Pair;
            }
            1 => {
                if joker && card_count.contains_key(&'J') {
                    return Kind::Pair;
                }

                return Kind::High;
            }
            _ => unreachable!(),
        };
    }

    pub fn parse(text: &str, joker: bool) -> Hand {
        let kind = Self::parse_kind(text, joker);
        let chars = text.chars().collect::<Vec<_>>();
        return Hand(chars.deref().try_into().unwrap(), kind, joker);
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.1 != other.1 {
            return self.1.cmp(&other.1).reverse();
        }

        let Some((l, r)) = self.0.iter().zip(other.0.iter()).find(|(l, r)| l != r) else {
            return std::cmp::Ordering::Equal;
        };
        return get_value(*l, self.2).cmp(&get_value(*r, self.2));
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}

pub fn register_all(puzzles: &mut Puzzles) {
    puzzles.register(crate::util::Puzzle {
        name: "Camel Cards".to_string(),
        year: 2023,
        day: 7,
        solver: Box::new(|input| {
            let mut sum = 0;

            let mut bets = Vec::new();
            for line in input.text().lines() {
                let split = line.split_ascii_whitespace().collect::<Vec<_>>();
                let hand = Hand::parse(split[0], false);
                let bet = split[1].parse::<u32>()?;
                bets.push((hand, bet));
            }

            bets.sort_unstable_by_key(|b| b.0.clone());

            for (i, bet) in bets.iter().enumerate() {
                sum += ((i + 1) as u32) * bet.1;
            }

            Ok(sum.to_string())
        }),
    });
    puzzles.register(crate::util::Puzzle {
        name: "Part Two".to_string(),
        year: 2023,
        day: 7,
        solver: Box::new(|input| {
            let mut sum = 0;

            let mut bets = Vec::new();
            for line in input.text().lines() {
                let split = line.split_ascii_whitespace().collect::<Vec<_>>();
                let hand = Hand::parse(split[0], true);
                let bet = split[1].parse::<u32>()?;
                bets.push((hand, bet));
            }

            bets.sort_unstable_by_key(|b| b.0.clone());

            for (i, bet) in bets.iter().enumerate() {
                sum += ((i + 1) as u32) * bet.1;
            }

            Ok(sum.to_string())
        }),
    });
}
