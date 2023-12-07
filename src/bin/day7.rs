#![allow(unused)]

use std::cmp::Ordering;

use hand_bid::{parse_hands, HandBid};

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/day7.txt").unwrap();
    println!("Answer: {}", solve(&input));
}

mod hand_bid {
    use itertools::Itertools;

    use hand::Hand;

    pub fn parse_hands(input: &str) -> Vec<HandBid> {
        input
            .lines()
            .map(|line| {
                let (cards, bet) = line.split_once(' ').unwrap();
                let hand = hand::Hand::new(cards.chars().collect_vec().try_into().unwrap());
                let bet = bet.parse().unwrap();
                HandBid { hand, bet }
            })
            .collect_vec()
    }

    #[derive(Debug, Eq, Clone)]
    pub struct HandBid {
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

    mod hand {
        use itertools::Itertools;
        use std::cmp::Ordering;

        /// Valid card characters, ordered by their value
        const VALID_CARDS: [char; 13] = [
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
        ];

        #[derive(Debug, Eq, Clone)]
        pub(crate) struct Hand {
            pub(crate) cards: [char; 5],
            pub(crate) hand_type: HandType,
        }

        impl Ord for Hand {
            fn cmp(&self, other: &Self) -> Ordering {
                match self.hand_type.cmp(&other.hand_type) {
                    Ordering::Less => Ordering::Less,
                    Ordering::Greater => Ordering::Greater,
                    Ordering::Equal => {
                        for (self_card, other_card) in self.cards.iter().zip(other.cards) {
                            let self_position =
                                VALID_CARDS.iter().position(|c| self_card == c).unwrap();
                            let other_position =
                                VALID_CARDS.iter().position(|c| other_card == *c).unwrap();
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

        // Equal hands not allowed
        impl PartialEq for Hand {
            fn eq(&self, other: &Self) -> bool {
                false
            }
        }

        impl Hand {
            pub(crate) fn new(mut cards: [char; 5]) -> Self {
                let hand_type = parse_hand_type(&cards);

                Self { cards, hand_type }
            }
        }

        pub(crate) fn parse_hand_type(cards: &[char; 5]) -> HandType {
            let count_map = cards.iter().counts();
            let mut card_counts = count_map.into_values().collect_vec();
            card_counts.sort();
            card_counts.reverse();

            let highest = *card_counts.first().unwrap_or(&0);
            let second_heighest = *card_counts.get(1).unwrap_or(&0);
            match (highest, second_heighest) {
                (5, 0) => HandType::FiveOfAKind,
                (4, 1) => HandType::FourOfAKind,
                (3, 2) => HandType::FullHouse,
                (3, _) => HandType::ThreeOfAKind,
                (2, 2) => HandType::TwoPair,
                (2, _) => HandType::OnePair,
                _ => HandType::HighCard,
            }
        }

        impl PartialOrd for Hand {
            fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

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
}

fn solve(input: &str) -> usize {
    let mut hands = parse_hands(input);
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
    assert_eq!(solve(input), 6440)
}
