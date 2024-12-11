use std::collections::HashMap;
use std::fs;

fn one(input: String) {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Space {
        Rock,
        Wall,
        Free,
    }
    use Space::{Free, Rock, Wall};

    fn roll_north(platform: &mut Vec<Vec<Space>>) {
        let mut did_move = true;

        let height = platform.len();
        let width = platform[0].len();
        while did_move {
            did_move = false;
            for y in 1..height {
                for x in 0..width {
                    if Rock == platform[y][x] && Free == platform[y - 1][x] {
                        platform[y - 1][x] = Rock;
                        platform[y][x] = Free;
                        did_move = true
                    }
                }
            }
        }
    }

    fn calculate_load(platform: &Vec<Vec<Space>>) -> usize {
        let mut load: usize = 0;

        let height = platform.len();
        let width = platform[0].len();
        for l in 1..height + 1 {
            for x in 0..width {
                if Rock == platform[height - l][x] {
                    load += l;
                }
            }
        }
        load
    }

    let mut platform: Vec<Vec<Space>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'O' => Rock,
                    '#' => Wall,
                    '.' => Free,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    roll_north(&mut platform);
    println!("{}", calculate_load(&platform));
}

fn two(input: String) {
    const SPINS: u32 = 1000000000;

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    enum Space {
        Rock,
        Wall,
        Free,
    }
    use Space::{Free, Rock, Wall};

    /// exactly one of the values should be `1` and the others `0`
    fn roll(platform: &mut Vec<Vec<Space>>, north: usize, west: usize, south: usize, east: usize) {
        let mut did_move = true;

        let height = platform.len();
        let width = platform[0].len();

        while did_move {
            did_move = false;
            for y in north..height - south {
                for x in west..width - east {
                    if Rock == platform[y][x]
                        && Free == platform[y - north + south][x - west + east]
                    {
                        platform[y - north + south][x - west + east] = Rock;
                        platform[y][x] = Free;
                        did_move = true
                    }
                }
            }
        }
    }

    fn spin(platform: &mut Vec<Vec<Space>>) {
        roll(platform, 1, 0, 0, 0);
        roll(platform, 0, 1, 0, 0);
        roll(platform, 0, 0, 1, 0);
        roll(platform, 0, 0, 0, 1);
    }

    fn calculate_load_north(platform: &Vec<Vec<Space>>) -> usize {
        let mut load: usize = 0;

        let height = platform.len();
        let width = platform[0].len();
        for l in 1..height + 1 {
            for x in 0..width {
                if Rock == platform[height - l][x] {
                    load += l;
                }
            }
        }
        load
    }

    let mut platform: Vec<Vec<Space>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'O' => Rock,
                    '#' => Wall,
                    '.' => Free,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let mut cache: HashMap<Vec<Vec<Space>>, u32> = HashMap::new();
    let mut start_from = 0;
    for i in 1..SPINS + 1 {
        spin(&mut platform);

        if let Some(prev) = cache.get(&platform) {
            let cycle = i - prev;
            start_from = SPINS - ((SPINS - prev) % cycle);
            break;
        } else {
            cache.insert(platform.clone(), i);
        }
    }
    for _ in start_from..SPINS {
        spin(&mut platform);
    }
    println!("{}", calculate_load_north(&platform));
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
