use std::{cmp::Ordering, collections::HashSet};

fn open_input(filename: &str) -> String {
    let content = std::fs::read_to_string(filename).expect(&format!("Could not open {filename}"));
    content
}

#[derive(Debug)]
enum Hand {
    // 1 char, 1 x 5
    FiveOfKind(String, u64),
    // 2 char, 1 x 4, 1
    FourOfKind(String, u64),
    // 2 char - 1 x 3, 1 x 2
    FullHouse(String, u64),
    // 3 char - 1 x 3, 1, 1
    ThreeOfKind(String, u64),
    // 3 char - 1 x 2, 1 x 2, 1
    TwoPair(String, u64),
    // 4 char - 1 x 2, 1, 1, 1
    OnePair(String, u64),
    // 5 char - 1, 1, 1, 1, 1
    HighCard(String, u64),
}

fn second_ordering(h1: String, h2: String) -> Ordering {
    for (c1, c2) in h1.chars().zip(h2.chars()) {
        match c1 {
            'A' => match c2 {
                'A' => (),
                _ => return Ordering::Less,
            },
            'K' => match c2 {
                'A' => return Ordering::Greater,
                'K' => (),
                _ => return Ordering::Less,
            },
            'Q' => match c2 {
                'A' | 'K' => return Ordering::Greater,
                'Q' => (),
                _ => return Ordering::Less,
            },

            'J' => match c2 {
                'A' | 'K' | 'Q' => return Ordering::Greater,
                'J' => (),
                _ => return Ordering::Less,
            },

            'T' => match c2 {
                'A' | 'K' | 'Q' | 'J' => return Ordering::Greater,
                'T' => (),
                _ => return Ordering::Less,
            },
            '9' | '8' | '7' | '6' | '5' | '4' | '3' | '2' | '1' => match c2 {
                'A' | 'K' | 'Q' | 'J' | 'T' => return Ordering::Greater,
                _ => match c2.cmp(&c1) {
                    Ordering::Less => return Ordering::Less,
                    Ordering::Equal => (),
                    Ordering::Greater => return Ordering::Greater,
                },
            },
            _ => (),
        }
    }
    // doesn't matter
    Ordering::Greater
}

fn part1() {
    let content = open_input("bin/day7/input.txt");
    let mut hands = content
        .lines()
        .map(|l| {
            let hand = l.split(" ").nth(0).expect("No fst");
            let rank = l
                .split(" ")
                .nth(1)
                .expect("no snd")
                .parse::<u64>()
                .expect("Could not parse rank to int");
            let unique_cards = hand.chars().collect::<HashSet<char>>();
            match unique_cards.len() {
                1 => Hand::FiveOfKind(hand.to_string(), rank),
                2 => match unique_cards
                    .into_iter()
                    .map(|c| hand.matches(&c.to_string()).count())
                    .max()
                    .expect("No max")
                {
                    4 => Hand::FourOfKind(hand.to_string(), rank),
                    3 => Hand::FullHouse(hand.to_string(), rank),
                    _ => panic!("No matching case for 2 unique cards"),
                },
                3 => match unique_cards
                    .into_iter()
                    .map(|c| hand.matches(&c.to_string()).count())
                    .max()
                    .expect("No max")
                {
                    3 => Hand::ThreeOfKind(hand.to_string(), rank),
                    2 => Hand::TwoPair(hand.to_string(), rank),
                    _ => panic!("No matching case for 3 unique cards"),
                },
                4 => Hand::OnePair(hand.to_string(), rank),
                5 => Hand::HighCard(hand.to_string(), rank),
                _ => panic!("More than 5 unique cards"),
            }
        })
        .collect::<Vec<Hand>>();

    hands.sort_by(|a, b| match a {
        Hand::FiveOfKind(h1, _) => match b {
            Hand::FiveOfKind(h2, _) => second_ordering(h1.to_string(), h2.to_string()),
            _ => std::cmp::Ordering::Less,
        },
        Hand::FourOfKind(h1, _) => match b {
            Hand::FiveOfKind(_, _) => std::cmp::Ordering::Greater,
            Hand::FourOfKind(h2, _) => second_ordering(h1.to_string(), h2.to_string()),
            _ => std::cmp::Ordering::Less,
        },
        Hand::FullHouse(h1, _) => match b {
            Hand::FiveOfKind(_, _) | Hand::FourOfKind(_, _) => std::cmp::Ordering::Greater,
            Hand::FullHouse(h2, _) => second_ordering(h1.to_string(), h2.to_string()),
            _ => std::cmp::Ordering::Less,
        },
        Hand::ThreeOfKind(h1, _) => match b {
            Hand::FiveOfKind(_, _) | Hand::FourOfKind(_, _) | Hand::FullHouse(_, _) => {
                std::cmp::Ordering::Greater
            }
            Hand::ThreeOfKind(h2, _) => second_ordering(h1.to_string(), h2.to_string()),
            _ => std::cmp::Ordering::Less,
        },
        Hand::TwoPair(h1, _) => match b {
            Hand::FiveOfKind(_, _)
            | Hand::FourOfKind(_, _)
            | Hand::FullHouse(_, _)
            | Hand::ThreeOfKind(_, _) => std::cmp::Ordering::Greater,
            Hand::TwoPair(h2, _) => second_ordering(h1.to_string(), h2.to_string()),
            _ => std::cmp::Ordering::Less,
        },
        Hand::OnePair(h1, _) => match b {
            Hand::FiveOfKind(_, _)
            | Hand::FourOfKind(_, _)
            | Hand::FullHouse(_, _)
            | Hand::ThreeOfKind(_, _)
            | Hand::TwoPair(_, _) => std::cmp::Ordering::Greater,
            Hand::OnePair(h2, _) => second_ordering(h1.to_string(), h2.to_string()),
            _ => std::cmp::Ordering::Less,
        },
        Hand::HighCard(h1, _) => match b {
            Hand::FiveOfKind(_, _)
            | Hand::FourOfKind(_, _)
            | Hand::FullHouse(_, _)
            | Hand::ThreeOfKind(_, _)
            | Hand::TwoPair(_, _)
            | Hand::OnePair(_, _) => std::cmp::Ordering::Greater,
            Hand::HighCard(h2, _) => second_ordering(h1.to_string(), h2.to_string()),
            _ => std::cmp::Ordering::Less,
        },
    });
    let hands_len = hands.len();
    let res = hands
        .into_iter()
        .enumerate()
        .map(|(idx, h)| match h {
            Hand::FiveOfKind(_, r)
            | Hand::FourOfKind(_, r)
            | Hand::FullHouse(_, r)
            | Hand::ThreeOfKind(_, r)
            | Hand::TwoPair(_, r)
            | Hand::OnePair(_, r)
            | Hand::HighCard(_, r) => (hands_len - idx) as u64 * r,
        })
        .sum::<u64>();
    println!("{res}");
}

fn main() {
    part1();
}
