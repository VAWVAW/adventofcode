use std::{collections::HashMap, fs};

fn one(input: String) {
    #[derive(Debug)]
    struct Line {
        hand: (u8, [u8; 5]),
        bid: u16,
    }
    impl PartialEq for Line {
        fn eq(&self, other: &Self) -> bool {
            self.hand == other.hand
        }
    }
    impl Eq for Line {}
    impl PartialOrd for Line {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for Line {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.hand.cmp(&other.hand)
        }
    }
    impl From<&str> for Line {
        fn from(value: &str) -> Self {
            fn hand_value(value: [u8; 5]) -> (u8, [u8; 5]) {
                let mut card_counts = HashMap::new();
                for v in value.iter() {
                    let count = if let Some(i) = card_counts.get(v) {
                        i + 1
                    } else {
                        1
                    };
                    card_counts.insert(v, count);
                }

                match card_counts.keys().count() {
                    1 => (6, value),
                    2 => {
                        if *card_counts.values().max().unwrap() == 4 {
                            (5, value)
                        } else {
                            (4, value)
                        }
                    }
                    3 => {
                        if *card_counts.values().max().unwrap() == 3 {
                            (3, value)
                        } else {
                            (2, value)
                        }
                    }
                    4 => (1, value),
                    5 => (0, value),
                    _ => unreachable!(),
                }
            }

            let (hand_str, bid_str) = value.split_once(" ").unwrap();
            let mut hand_iter = hand_str.chars().map(|c| {
                if c.is_ascii_digit() {
                    c.to_digit(10).unwrap() as u8
                } else {
                    match c {
                        'T' => 10,
                        'J' => 11,
                        'Q' => 12,
                        'K' => 13,
                        'A' => 14,

                        _ => unreachable!(),
                    }
                }
            });
            let hand_cards = [
                hand_iter.next().unwrap(),
                hand_iter.next().unwrap(),
                hand_iter.next().unwrap(),
                hand_iter.next().unwrap(),
                hand_iter.next().unwrap(),
            ];
            Self {
                hand: hand_value(hand_cards),
                bid: bid_str.parse().unwrap(),
            }
        }
    }

    let mut lines: Vec<Line> = input.lines().map(|line| line.into()).collect();
    lines.as_mut_slice().sort_unstable();
    let winning: u64 = lines
        .iter()
        .enumerate()
        .map(|(i, line)| ((i + 1) * line.bid as usize) as u64)
        .sum();

    println!("{winning}")
}

fn two(input: String) {
    #[derive(Debug)]
    struct Line {
        hand: (u8, [u8; 5]),
        bid: u16,
    }
    impl PartialEq for Line {
        fn eq(&self, other: &Self) -> bool {
            self.hand == other.hand
        }
    }
    impl Eq for Line {}
    impl PartialOrd for Line {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for Line {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.hand.cmp(&other.hand)
        }
    }
    impl From<&str> for Line {
        fn from(value: &str) -> Self {
            fn hand_value(value: [u8; 5]) -> (u8, [u8; 5]) {
                let mut combinations = Vec::new();

                for joker_value in 2..15 {
                    let mut card_counts: HashMap<u8, u8> = HashMap::new();
                    for v in value.iter() {
                        let count = if let Some(i) = card_counts.get(v) {
                            i + 1
                        } else {
                            1
                        };
                        card_counts.insert(*v, count);
                    }

                    if let Some(count) = card_counts.remove(&0) {
                        card_counts.insert(
                            joker_value,
                            count + card_counts.get(&joker_value).unwrap_or(&0),
                        );
                    }

                    let hand = match card_counts.keys().count() {
                        1 => (6, value),
                        2 => {
                            if *card_counts.values().max().unwrap() == 4 {
                                (5, value)
                            } else {
                                (4, value)
                            }
                        }
                        3 => {
                            if *card_counts.values().max().unwrap() == 3 {
                                (3, value)
                            } else {
                                (2, value)
                            }
                        }
                        4 => (1, value),
                        5 => (0, value),
                        _ => unreachable!(),
                    };
                    combinations.push(hand);
                }

                combinations.into_iter().max().unwrap()
            }

            let (hand_str, bid_str) = value.split_once(" ").unwrap();
            let mut hand_iter = hand_str.chars().map(|c| {
                if c.is_ascii_digit() {
                    c.to_digit(10).unwrap() as u8
                } else {
                    match c {
                        'T' => 10,
                        'J' => 0,
                        'Q' => 12,
                        'K' => 13,
                        'A' => 14,

                        _ => unreachable!(),
                    }
                }
            });
            let hand_cards = [
                hand_iter.next().unwrap(),
                hand_iter.next().unwrap(),
                hand_iter.next().unwrap(),
                hand_iter.next().unwrap(),
                hand_iter.next().unwrap(),
            ];
            Self {
                hand: hand_value(hand_cards),
                bid: bid_str.parse().unwrap(),
            }
        }
    }

    let mut lines: Vec<Line> = input.lines().map(|line| line.into()).collect();
    lines.as_mut_slice().sort_unstable();
    let winning: u64 = lines
        .iter()
        .enumerate()
        .map(|(i, line)| ((i + 1) * line.bid as usize) as u64)
        .sum();

    println!("{winning}")
}

fn main() {
    let (execute_first, file_name) = match std::env::args()
        .nth(1)
        .unwrap_or("test1".to_owned())
        .as_str()
    {
        "1" => (true, "data.txt"),
        "2" => (false, "data.txt"),
        "t1" => (true, "data_test1.txt"),
        "t2" => (false, "data_test2.txt"),
        _ => (true, "data_test1.txt"),
    };
    let input = fs::read_to_string(file_name).unwrap();

    if execute_first {
        one(input);
    } else {
        two(input);
    }
}
