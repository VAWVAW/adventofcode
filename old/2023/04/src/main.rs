use std::fs;

fn one(input: String) {
    let mut points = 0;

    for card in input.lines() {
        let (winning, contained) = card.split_once(':').unwrap().1.split_once('|').unwrap();
        let winning: Vec<i32> = winning
            .trim()
            .split(' ')
            .filter(|s| s.len() > 0)
            .map(|s| i32::from_str_radix(s, 10).unwrap())
            .collect();
        let contained: Vec<i32> = contained
            .trim()
            .split(' ')
            .filter(|s| s.len() > 0)
            .map(|s| i32::from_str_radix(s, 10).unwrap())
            .collect();

        let mut points_card = 1;

        contained
            .iter()
            .filter(|v| winning.contains(v))
            .for_each(|_| points_card *= 2);
        points += points_card / 2;
    }

    println!("{points}");
}

fn two(input: String) {
    fn get_or_fill(counts: &mut Vec<i32>, index: usize) -> i32 {
        if index >= counts.len() {
            counts.resize(index + 1, 1);
        }

        counts[index]
    }
    fn inc_or_fill(counts: &mut Vec<i32>, index: usize, value: i32) {
        if index >= counts.len() {
            counts.resize(index + 1, 1);
        }

        counts[index] += value
    }

    let mut points = 0;
    let mut counts = Vec::new();

    for (num, card) in input.lines().enumerate() {
        let (winning, contained) = card.split_once(':').unwrap().1.split_once('|').unwrap();
        let winning: Vec<i32> = winning
            .trim()
            .split(' ')
            .filter(|s| s.len() > 0)
            .map(|s| i32::from_str_radix(s, 10).unwrap())
            .collect();
        let contained: Vec<i32> = contained
            .trim()
            .split(' ')
            .filter(|s| s.len() > 0)
            .map(|s| i32::from_str_radix(s, 10).unwrap())
            .collect();

        let mut points_card = 0;
        let count_card = get_or_fill(&mut counts, num);

        contained
            .iter()
            .filter(|v| winning.contains(v))
            .for_each(|_| points_card += 1);

        for i in 0..points_card {
            inc_or_fill(&mut counts, num + i + 1, count_card);
        }
        points += count_card;
    }

    println!("{points}");
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
