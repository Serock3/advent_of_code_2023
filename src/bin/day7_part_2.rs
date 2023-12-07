#![allow(unused)]

use std::cmp::Ordering;

use hand_bid::HandBid;
use hand_type::HandType;
use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/day7.txt").unwrap();
    println!("Answer 1: {}", solve(&input));
}

/// Valid card characters, ordered by their value
const VALID_CARDS: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

#[derive(Debug, Eq, Clone)]
struct Hand {
    cards: [char; 5],
    hand_type: HandType,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                for (self_card, other_card) in self.cards.iter().zip(other.cards) {
                    let self_position = VALID_CARDS.iter().position(|c| self_card == c).unwrap();
                    let other_position = VALID_CARDS.iter().position(|c| other_card == *c).unwrap();
                    match self_position.cmp(&other_position) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Equal => {
                            continue;
                        }
                    }
                }
                panic!("Hands are of equal value. Not allowed.")
            }
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

impl Hand {
    fn new(mut cards: [char; 5]) -> Self {
        let joker_hand_type = parse_joker_hand_type(cards);

        Self {
            cards,
            hand_type: joker_hand_type,
        }
    }
}

fn parse_joker_hand_type(cards: [char; 5]) -> HandType {
    let binding = cards.iter().counts();
    let joker_count = binding.get(&'J').unwrap_or(&0);
    let mut card_counts = binding
        .iter()
        .filter_map(|(k, v)| if *k == &'J' { None } else { Some(v) })
        .collect_vec();
    card_counts.sort();
    card_counts.reverse();

    let highest_count = *card_counts.get(0).unwrap_or(&&0) + joker_count;
    if highest_count == 5 {
        HandType::FiveOfAKind
    } else if highest_count == 4 {
        HandType::FourOfAKind
    } else if highest_count == 3 {
        if *card_counts[1] == 2 {
            HandType::FullHouse
        } else {
            HandType::ThreeOfAKind
        }
    } else if highest_count == 2 {
        if *card_counts[1] == 2 {
            HandType::TwoPair
        } else {
            HandType::OnePair
        }
    } else {
        HandType::HighCard
    }
}

fn parse_hand_type(cards: [char; 5]) -> HandType {
    let binding = cards.iter().counts();
    let mut card_counts = binding.values().collect_vec();
    card_counts.sort();
    card_counts.reverse();

    if *card_counts[0] == 5 {
        HandType::FiveOfAKind
    } else if *card_counts[0] == 4 {
        HandType::FourOfAKind
    } else if *card_counts[0] == 3 {
        if *card_counts[1] == 2 {
            HandType::FullHouse
        } else {
            HandType::ThreeOfAKind
        }
    } else if *card_counts[0] == 2 {
        if *card_counts[1] == 2 {
            HandType::TwoPair
        } else {
            HandType::OnePair
        }
    } else {
        HandType::HighCard
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

mod hand_type {
    #[derive(Debug, Eq, Ord, PartialOrd, PartialEq, Clone)]
    pub(crate) enum HandType {
        HighCard,
        OnePair,
        TwoPair,
        ThreeOfAKind,
        FullHouse,
        FourOfAKind,
        FiveOfAKind,
    }
}

mod hand_bid {
    use super::Hand;

    #[derive(Debug, Eq, Clone)]
    pub(crate) struct HandBid {
        pub(crate) hand: Hand,
        pub bet: u32,
    }

    impl Ord for HandBid {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.hand.cmp(&other.hand)
        }
    }

    impl PartialOrd for HandBid {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.hand.partial_cmp(&other.hand)
        }
    }

    impl PartialEq for HandBid {
        fn eq(&self, other: &Self) -> bool {
            self.hand == other.hand
        }
    }
}

fn solve(input: &str) -> usize {
    let mut hands = input
        .lines()
        .map(|line| {
            let (cards, bet) = line.split_once(' ').unwrap();
            let hand = Hand::new(cards.chars().collect_vec().try_into().unwrap());
            let bet = bet.parse().unwrap();
            HandBid { hand, bet }
        })
        .collect_vec();
    hands.sort();
    // dbg!(hands.clone());
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bet as usize)
        .sum()
}

#[test]
fn test_example() {
    let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    assert_eq!(solve(input), 5905)
}
