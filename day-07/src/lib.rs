use std::cmp::Ordering;

use counter::Counter;

pub type Card = char;

const CARDS: [Card; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

fn card_priority(card: &Card) -> usize {
    CARDS.iter().position(|c| c == card).unwrap()
}

fn cmp_card(card1: &Card, card2: &Card) -> Ordering {
    card_priority(card1).cmp(&card_priority(card2))
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
enum HandKind {
    Five,
    Four,
    FullHouse,
    Three,
    TwoPairs,
    OnePair,
    HighCard,
}

use HandKind::*;

impl HandKind {
    fn priority(&self) -> u32 {
        match self {
            Five => 6,
            Four => 5,
            FullHouse => 4,
            Three => 3,
            TwoPairs => 2,
            OnePair => 1,
            HighCard => 0,
        }
    }
}

impl Ord for HandKind {
    fn cmp(&self, other: &HandKind) -> Ordering {
        self.priority().cmp(&other.priority())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub struct Hand {
    kind: HandKind,
    cards: Vec<Card>,
    pub bid: u32,
}

impl Hand {
    pub fn new(cards: Vec<Card>, bid: u32) -> Hand {
        let occs = cards
            .clone()
            .into_iter()
            .collect::<Counter<Card>>()
            .most_common();
        let kind = match occs[0].1 {
            5 => Five,
            4 => Four,
            3 => {
                if occs[1].1 == 2 {
                    FullHouse
                } else {
                    Three
                }
            }
            2 => {
                if occs[1].1 == 2 {
                    TwoPairs
                } else {
                    OnePair
                }
            }
            1 => HighCard,
            _ => panic!(),
        };

        Hand { kind, cards, bid }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        match self.kind.priority().cmp(&other.kind.priority()) {
            Ordering::Equal => {
                for i in 0..5 {
                    let ord = cmp_card(&self.cards[i], &other.cards[i]);
                    if ord != Ordering::Equal {
                        return ord;
                    }
                }
                Ordering::Equal
            }
            o => o,
        }
    }
}

impl<'a> From<&'a str> for Hand {
    fn from(value: &'a str) -> Hand {
        let mut split = value.split_whitespace();
        Hand::new(
            split.next().unwrap().chars().collect(),
            split.next().unwrap().parse().unwrap(),
        )
    }
}

pub fn parse_hands(input: &str) -> Vec<Hand> {
    input.lines().map(Hand::from).collect()
}

#[test]
fn test_hand_ordering() {
    let hands = vec![
        Hand::from("32T3K 0"),
        Hand::from("KTJJT 0"),
        Hand::from("KK677 0"),
        Hand::from("T55J5 0"),
        Hand::from("QQQJA 0"),
        Hand::from("22225 0"),
        Hand::from("22227 0"),
    ];

    for i in 0..hands.len() {
        for j in 0..hands.len() {
            assert_eq!(i.cmp(&j), hands[i].cmp(&hands[j]));
        }
    }
}
