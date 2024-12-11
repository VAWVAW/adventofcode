use std::{collections::HashMap, fs};

fn one(input: String) {
    type Springs = Vec<Option<bool>>;

    fn check_row_validity(springs: &Springs, broken: &Vec<u8>) -> bool {
        if springs.iter().any(Option::is_none) {
            panic!("can only check solved combinations");
        }

        let mut spring_iter = springs.iter();
        let mut last = spring_iter.next();
        for num in broken {
            while Some(&Some(true)) == last {
                last = spring_iter.next();
            }
            for _ in 0..*num {
                if Some(&Some(false)) != last {
                    return false;
                }
                last = spring_iter.next();
            }
            if Some(&Some(false)) == last {
                return false;
            }
        }
        while let Some(&Some(spring)) = last {
            if !spring {
                return false;
            }
            last = spring_iter.next();
        }
        true
    }
    fn parse_row(springs: &Springs, broken: &Vec<u8>) -> u32 {
        let unknown: Vec<usize> = springs
            .iter()
            .copied()
            .enumerate()
            .filter_map(|(i, spring)| if spring.is_none() { Some(i) } else { None })
            .collect();

        let mut valid_combinations = 0;
        let mut current_springs = springs.clone();
        for combination in 0..2u32.pow(unknown.len() as u32) {
            for (i, i_unknown) in unknown.iter().enumerate() {
                let v = (combination >> i) % 2 == 1;
                current_springs[*i_unknown] = Some(v);
            }

            if check_row_validity(&current_springs, broken) {
                valid_combinations += 1;
            }
        }

        valid_combinations
    }
    let rows: Vec<(Springs, Vec<u8>)> = input
        .lines()
        .map(|line| {
            let (springs, broken) = line.split_once(' ').unwrap();
            let springs = springs
                .chars()
                .map(|c| match c {
                    '#' => Some(false),
                    '.' => Some(true),
                    _ => None,
                })
                .collect();
            let broken = broken
                .split(',')
                .map(str::parse)
                .map(Result::unwrap)
                .collect();
            (springs, broken)
        })
        .collect();

    let combinations: u32 = rows
        .iter()
        .map(|(springs, broken)| parse_row(springs, broken))
        .sum();

    println!("{combinations}");
}

// based on https://github.com/Gobbel2000/advent-of-code aoc2023/src/bin/day12.rs
fn two(input: String) {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Spring {
        Broken,
        Working,
        Unknown,
    }
    use Spring::{Broken, Unknown, Working};

    #[derive(Debug, Clone)]
    struct Row {
        springs: Vec<Spring>,
        ranges: Vec<u8>,
        cache: HashMap<(usize, usize), u64>,
    }

    impl Row {
        fn new(springs: Vec<Spring>, ranges: Vec<u8>) -> Self {
            Self {
                springs,
                ranges,
                cache: HashMap::new(),
            }
        }
        fn unfold(mut self, n: u8) -> Self {
            let mut new_springs = Vec::new();
            let mut new_ranges = Vec::new();

            (0..n).for_each(|_| {
                new_ranges.append(&mut self.ranges.clone());
                new_springs.append(&mut self.springs.clone());
                new_springs.push(Unknown);
            });

            new_springs.pop();

            self.ranges = new_ranges;
            self.springs = new_springs;

            self
        }

        fn get_combinations(&mut self) -> u64 {
            self.recursion_stepper(0, 0)
        }
        fn recursion_stepper(&mut self, spring_index: usize, range_index: usize) -> u64 {
            if let Some(combinations) = self.cache.get(&(spring_index, range_index)) {
                *combinations
            } else {
                let combinations = self.recursion_step(spring_index, range_index);
                self.cache.insert((spring_index, range_index), combinations);
                combinations
            }
        }
        fn recursion_step(&mut self, spring_index: usize, range_index: usize) -> u64 {
            if spring_index >= self.springs.len() {
                if range_index >= self.ranges.len() {
                    return 1;
                } else {
                    return 0;
                }
            }

            if range_index >= self.ranges.len() {
                if self.springs[spring_index..]
                    .iter()
                    .any(|spring| spring == &Broken)
                {
                    return 0;
                }
                return 1;
            }

            let spring = self.springs[spring_index];

            if spring == Working {
                return self.recursion_stepper(spring_index + 1, range_index);
            }

            // spring is broken or unknown
            let range = self.ranges[range_index];
            let mut combinations = if self.can_advance(spring_index, range) {
                self.recursion_stepper(spring_index + range as usize + 1, range_index + 1)
            } else {
                0
            };

            if spring == Unknown {
                combinations += self.recursion_stepper(spring_index + 1, range_index);
            }

            combinations
        }

        fn can_advance(&self, spring_index: usize, range: u8) -> bool {
            let end = spring_index + range as usize;
            if end > self.springs.len() {
                false
            } else {
                !self.springs[spring_index..end]
                    .iter()
                    .any(|spring| spring == &Working)
                    && self.springs.get(end) != Some(&Broken)
            }
        }
    }

    let rows: Vec<Row> = input
        .lines()
        .map(|line| {
            let (springs_s, broken_s) = line.split_once(' ').unwrap();
            let springs = springs_s
                .chars()
                .map(|c| match c {
                    '#' => Broken,
                    '.' => Working,
                    '?' => Unknown,
                    _ => unreachable!(),
                })
                .collect();
            let ranges = broken_s
                .split(',')
                .map(str::parse)
                .map(Result::unwrap)
                .collect();
            Row::new(springs, ranges).unfold(5)
        })
        .collect();

    let combinations: u64 = rows.into_iter().map(|mut row| row.get_combinations()).sum();

    println!("{combinations}");
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
        _ => (false, "data_test1.txt"),
    };
    let input = fs::read_to_string(file_name).unwrap();

    if execute_first {
        one(input);
    } else {
        two(input);
    }
}
